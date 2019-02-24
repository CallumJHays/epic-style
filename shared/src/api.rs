use serde::{Serialize, Deserialize};

pub trait Request<'a>: Serialize + Deserialize<'a> {
    fn url() -> String;
}

pub trait Handler<'a, R> where R: Request<'a> {
    // TODO: Make this more like 'impl Responder'
    // fn handle(req: R) -> Box<R>
}