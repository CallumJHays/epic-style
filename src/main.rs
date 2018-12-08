extern crate actix_web;
extern crate sentry;
extern crate sentry_actix;
extern crate listenfd;

use std::env;

use sentry_actix::SentryMiddleware;
use actix_web::{server, App, HttpRequest, Responder};
use listenfd::ListenFd;

fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!"
}

fn main() {
    let _guard = sentry::init("https://cd524f0a67cc4783ab3bcb4114e3b73d@sentry.io/1340081");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::panic::register_panic_handler();
    
    
    let server = server::new(|| App::new()
        .middleware(SentryMiddleware::new())
        .resource("/", |r| r.f(index))
    );
    
    let mut listenfd = ListenFd::from_env();
    let server = if let Ok(Some(cargo_watch)) = listenfd.take_tcp_listener(0) {
        // let this server listen to development updates for hot-reloading
        server.listen(cargo_watch)
    } else {
        // or run it on production
        server.bind("0.0.0.0:80").unwrap()
    };
    
    server.run();
}