#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hehe {}

impl Hehe {
    pub fn new() -> Self {
        Hehe {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Msg {
    DoIt,
}

component! {
    type Self = Hehe;
    
    create(props: Hehe, link) => {
        props
    }
    
    view(&self) => {
        html! {
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
    
    update(&mut self, msg: Msg) => {
        true
    }
}