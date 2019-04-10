#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
extern crate serde_json;
extern crate shared;
extern crate yew;

use stdweb::web::{IParentNode, document};
use yew::app::App;
use shared::config::Config;
use shared::components::hehe;
use std::collections::HashMap;

fn main() {}
// don't do anything yet,..

// let the templated js call this function with the init config
#[js_export]
pub fn init(config: Config) {
    yew::initialize();
    let doc = document();
    for mount_id in config.mount_ids {
        let component: App<hehe::Model> = App::new();
        let mount_div = doc.query_selector(&format!("#{}", mount_id))
            .expect("Internal `stdweb` SyntaxError")
            .expect(&format!("Mount id {} not found in document", mount_id));
        component.mount(mount_div);
    }
    yew::run_loop();
}
