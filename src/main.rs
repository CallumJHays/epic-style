#[macro_use]
extern crate horrorshow;
extern crate failure;
extern crate actix_web;
extern crate sentry;
extern crate sentry_actix;
extern crate listenfd;
extern crate dotenv;
extern crate shared;
extern crate indoc;
extern crate itertools;
extern crate serde;
extern crate serde_json;

mod pages;
mod templates;

use std::env;
use sentry_actix::SentryMiddleware;
use actix_web::{server, App, fs};
use listenfd::ListenFd;

use crate::pages::index::index;

fn setup_sentry() -> Option<sentry::internals::ClientInitGuard> {
    if let Ok(sentry_endpoint) = dotenv::var("SENTRY_ENDPOINT") {
        let _guard = sentry::init(sentry_endpoint);
        env::set_var("RUST_BACKTRACE", "1");
        sentry::integrations::panic::register_panic_handler();
        Some(_guard)
    } else {
        None
    }
}

fn main() {
    // this will set up sentry if it works. ignore if it doesnt
    let _guard = setup_sentry();
    
    let server = server::new(|| App::new()
        .middleware(SentryMiddleware::new())
        .resource("/", |r| r.f(index))
        .handler("/",
            fs::StaticFiles::new("./static")
                .unwrap()
                .show_files_listing())
    );
    
    let mut listenfd = ListenFd::from_env();
    let server = if let Ok(Some(cargo_watch)) = listenfd.take_tcp_listener(0) {
        // let this server listen to development updates for hot-reloading
        server.listen(cargo_watch)
    } else { server };
    
    server.bind(
        format!("0.0.0.0:{}", env::var("PORT")
            .unwrap_or("8000".to_string()))
    ).unwrap().run();
}