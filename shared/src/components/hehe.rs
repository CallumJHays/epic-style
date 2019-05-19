#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hehe;

impl Hehe {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Msg {
    DoIt,
}

component! {
    type Self = Hehe;

    create(props: Hehe, _link) => {
        // instantiation proptype that work with both horrorshow
        // and yew's jsx-like type-safe templates - its black magic!
        props
    }

    view(&self) => {
        html! {
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }

    update(&mut self, _msg: Msg) => {
        true  // trigger view?
    }
}
