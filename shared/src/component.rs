// using this crate's functionally equivalent code, this crate
// wraps `serde` to overcome a slightly annoying issue when dealing with
// using traits for trait objects - when boxing them
// eg: Box<IsomorphicComponent> => `Compiler E

use erased_serde::Serialize;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json;
use std::collections::HashMap;
use std::fmt;

use crate::components::hehe::Hehe;

pub trait Isomorphic: Serialize {
    // this should really be a static method, however - to make this a serializable trait object,
    // no static methods should be present on the object (this requires a refactor in the rust compiler to fix,
    // and is currently being worked on).

    // implementations of this method should NOT rely on the contents of &self for its definition.
    fn _type_name(&self) -> &'static str;
}

// makes the serialize/deserialize functions available again thanks to erased_serde! as a temp fix
serialize_trait_object!(Isomorphic);

#[derive(Serialize)]
pub struct IsoWrapper {
    type_name: &'static str,
    box_model: BoxModel,
}

#[derive(Serialize)]
pub struct BoxModel(Box<dyn Isomorphic>);

unsafe fn get_type_name<I: Isomorphic>() -> &'static str {
    // the pointer shouldnt matter - if it ends up being used then we screwed up
    let null_ptr = 0 as *const I;

    return I::_type_name(&*null_ptr);
}

macro_rules! map_typename_to_deserializer {
    ($($type:ty),*) => {
        unsafe {
            let mut map:
                HashMap<&'static str, Box<Fn(serde_json::Value) ->
                Result<
                    IsoWrapper,
                    serde_json::Error
                >>> = HashMap::new();

            $(
                map.insert(
                    get_type_name::<$type>(),
                    Box::from(|val| {
                        Ok(serde_json::from_value::<IsoWrapper>(val)?)
                    })
                );
            )+

            map
        }
    }
}

impl<I: 'static + Isomorphic> From<Box<I>> for IsoWrapper {
    fn from(box_model: Box<I>) -> Self {
        Self {
            type_name: unsafe { get_type_name::<I>() },
            box_model: BoxModel(box_model),
        }
    }
}

struct IsoWrapperVisitor;

impl<'de> Visitor<'de> for IsoWrapperVisitor {
    type Value = IsoWrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "A config object as  specifiied above.")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let deserializer_map = map_typename_to_deserializer!(Hehe);
        let mut type_name: Option<String> = None;
        let mut model_json: Option<serde_json::Value> = None;

        while let Some((key, value)) = access.next_entry()? {
            let val: serde_json::Value = value;
            match key {
                "type_name" => {
                    type_name = Some(val.to_string());
                }
                "box_model" => {
                    model_json = Some(val);
                }
                _ => panic!("at the disco"),
            };
        }
        let iso_wrapper = deserializer_map
            .get(&type_name.expect("no type name").as_str())
            .unwrap()(model_json.expect("no model"))
        .expect("incorrect deserializer");

        Ok(iso_wrapper)
    }
}

impl<'de> Deserialize<'de> for IsoWrapper {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_map(IsoWrapperVisitor)
    }
}

pub type MountID = String;

pub struct MountDiv(MountID);

cfg_if! {
    if #[cfg(target_os="linux")]  // backend
    {
        use uuid_b64::UuidB64;
        use horrorshow::{RenderOnce, TemplateBuffer};
        use crate::config::Config;

        pub trait Mountable: 'static + Isomorphic + Sized {
            fn mount<I>(self, config: &mut Config,
                    init_msg: Option<Box<I>>) -> MountDiv
                    where I: 'static + Isomorphic + Sized {
                let id = UuidB64::new().to_string();

                config.components.insert(
                    id.clone(),
                    (
                        IsoWrapper::from(Box::new(self)),
                        init_msg.map(IsoWrapper::from)
                    )
                );

                MountDiv(id)
            }
        }

        impl RenderOnce for MountDiv {
            fn render_once(self, tmpl: &mut TemplateBuffer) {
                tmpl << html! {
                    div(id=self.0);
                };
            }
        }
    }
}

macro_rules! component {
    {
        type Self = $selfT:ty;

        create($props:ident: $propsT:ty, $componentlink:ident) =>
            $create:block

        view(&self) =>
            $view:block

        update(&mut self, $msg:ident: $msgT:ty) =>
            $update:block

    } => {

        use crate::component::Isomorphic;

        impl Isomorphic for $selfT {
            fn _type_name(&self) -> &'static str {
                return "$selfT";
            }
        }

        impl Isomorphic for $msgT {
            fn _type_name(&self) -> &'static str {
                return "$msgT";
            }
        }

        // only bundle the crates required just for each target separately
        cfg_if! {

            if #[cfg(target_os="linux")]  // backend
            {
                use crate::component::Mountable;

                impl Mountable for $selfT {}
            } else {
                use yew::prelude::*;

                // this is the main function mapping for yew
                impl Component for $selfT {
                    type Message = $msgT;
                    type Properties = $propsT;

                    fn create($props: Self::Properties,
                            $componentlink: ComponentLink<Self>) -> Self
                        $create

                    fn update(&mut self, $msg: Self::Message) -> ShouldRender
                        $update
                }

                impl Renderable<$selfT> for $selfT {
                    fn view(&self) -> Html<Self>
                        $view
                }

            }
        }
    }
}
