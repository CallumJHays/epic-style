use actix_web::{HttpRequest, Responder};


use crate::templates::{template::Template, page::Page};
use shared::{config::Config, components::hehe::Hehe, component::Mountable};

pub fn index(_req: &HttpRequest) -> impl Responder {
    let mut config = Config::new();
    
    Page::new(
        "EPICSTYLE",
        html! {
            div: Hehe::new().mount(&mut config, None);
            p: "dead memes society";
            img(src="https://i.kym-cdn.com/photos/images/newsfeed/001/300/333/a78.png");
        },
        config
    ).as_response()
}