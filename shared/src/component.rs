// using this crate's functionally equivalent code, this crate
// wraps `serde` to overcome a slightly annoying issue when dealing with
// using traits for trait objects - when boxing them
// eg: Box<IsomorphicComponent> => `Compiiler E

use std::fmt;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use erased_serde::Serialize;
use serde_json;

use crate::components::hehe::Hehe;

pub trait Isomorphic: Serialize {}

serialize_trait_object!(Isomorphic);

// describes 
#[derive(Serialize)]
pub struct IsoWrapper {
    type_name: &'static str,
    box_model: Box<Isomorphic>
}

pub trait IsoWrappable {
    fn _type_name() -> &'static str;
}


impl<I> From<Box<I>> for IsoWrapper where I: IsoWrappable {
    fn from(box_model: Box<I>) -> Self {
        Self {
            type_name: I::_type_name(),
            box_model
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
            where M: MapAccess<'de> {
        
        let components = [Hehe::_type_name()];
        let mut type_name: Option<&'static str> = None;
        let mut model_json: Option<serde_json::Value> = None;
        
        while let Some(key) = access.next_key()? {
            match key {
                "type_name" => {
                    let t = access.next_value()??;
                },
                "box_model" => {
                    model_json = access.next_value()?;
                }
            };
        }

        Ok(IsoWrapper {
            type_name: 
        })
    }
}

impl<'de> Deserialize<'de> for IsoWrapper {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where D: Deserializer<'de> {
            
        
    }
}

// this bridges the gap back to original serde's api with some compile-time
// safety for when certain APIs will not be available when using serde-lib
// serialize_trait_object!(
//     Isomorphic<
//         'a,
//         Serializer = CBORSerializer,
//         Deserializer = CBORDeserializer
//     >
// );

pub type MountID = String;

pub struct MountDiv(MountID);

cfg_if! {
    if #[cfg(target_os="linux")]  // backend
    {
        use uuid_b64::UuidB64;
        use horrorshow::{RenderOnce, TemplateBuffer};
        use crate::config::Config;
        
        pub trait Mountable: 'static + Isomorphic + Sized {
            fn mount(self, config: &mut Config,
                    init_msg: Option<Box<dyn Isomorphic>>) -> MountDiv {
                let id = UuidB64::new().to_string();
                
                config.components.insert(
                    id.clone(),
                    (
                        (Box::new(self) as Box<dyn Isomorphic>).into(),
                        init_msg.map(|msg| msg.into())
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
    
        use crate::component::{Isomorphic, IsoWrappable};
        
        impl Isomorphic for $selfT {}
        
        impl IsoWrappable for $selfT {
            fn _type_name() -> &'static str {
                "$selfT"
            }
        }
    
        // only bundle the crates required just for each target separately
        cfg_if! {

            if #[cfg(target_os="linux")]  // backend
            {
                use crate::component::Mountable;
                
                impl Mountable for $selfT {}
            }
            else
            {
                use yew::prelude::*;
                
                // this is the main function mapping for yew
                impl Component for $selfT {
                    type Message = $msgT;
                    type Properties = $propsT;
                
                    fn create($props: Self::Properties,
                            $componentlink: ComponentLink<Self>) -> Self
                        $create
                
                    fn update(&mut self, msg: Self::Message) -> ShouldRender
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
