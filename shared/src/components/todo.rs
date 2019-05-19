
use yew::services::storage::{StorageService, Area};

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    storage: StorageService,
    state: State
}

impl Todo {

    pub fn new() -> Self {
        Todo {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Msg {
    DoIt,
}

component! {
    type Self = Todo;
    
    create(props: Todo, link) => {
        // instantiation proptype that work with both horrorshow
        // and yew's jsx-like type-safe templates - its black magic!
        props
    }
    
    view(&self) => {
        html! {
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
    
    update(&mut self, msg: Msg) => {
        true  // trigger view?
    }
}
