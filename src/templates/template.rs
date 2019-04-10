use actix_web::{Responder, HttpRequest, HttpResponse};
use failure::Error;
use horrorshow::Template as BaseTemplate;

pub struct HTML<T>(T) where T: Template;

pub trait Template: BaseTemplate {
    fn as_response(self) -> HTML<Self> {
        HTML(self)
    }
}

impl<T> Responder for HTML<T> where T: Template {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>)
        -> Result<HttpResponse, Error> {
        
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(self.0.into_string()?)
        )
    }
}