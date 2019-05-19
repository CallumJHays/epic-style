extern crate serde_derive;
#[macro_use]
extern crate stdweb;
extern crate serde_json;
extern crate shared;
extern crate yew;

use stdweb::web::{IParentNode, INode, Element, document};
use yew::prelude::{App, Component, Renderable};
use shared::config::Config;
use shared::component::IsoWrapper;

fn main() {} // don't do anything yet,..

// let the templated js call this function with the init config
#[js_export]
pub fn init(config: Config) {
    yew::initialize();
    let doc = document();
    for (mount_id, (model, opt_init_msg)) in config.components {
        let app: App<IsoWrapper> = App::new();
        let mount_div = doc.query_selector(&format!("#{}", mount_id))
            .expect("Internal `stdweb` SyntaxError")
            .expect(&format!("Mount id {} not found in document", mount_id));
        _clear_element(&mount_div);
        let scope = app.mount(mount_div);
        scope.send_message()
    }
    yew::run_loop();
}

/// Removes anything from the given element.
fn _clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
    }
}
