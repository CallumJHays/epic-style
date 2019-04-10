#[macro_use]
extern crate cfg_if;

extern crate serde;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate erased_serde;

#[macro_use]
pub mod component;
pub mod config;
pub mod components;

// only bundle the crates required just for each target separately
cfg_if! {
    if #[cfg(target_os="linux")]  // backend
    {
        #[macro_use]
        extern crate horrorshow;
        extern crate actix_web;
        extern crate uuid_b64;
        
        pub mod api;
    }
    else  // frontend
    {
        #[macro_use]
        extern crate yew;
        #[macro_use]
        extern crate stdweb;
    }
}