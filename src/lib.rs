extern crate iron;
extern crate chrono;
extern crate bodyparser;
extern crate serde_json;
include!(concat!(env!("OUT_DIR"), "/request.rs"));
pub trait RequestHandler {
    fn handle_request(&self, &Request) -> Response;
}

struct IronHandler {
}
impl iron::middleware::Handler for IronHandler {
    fn handle(&self, req: &mut iron::Request) -> iron::prelude::IronResult<iron::prelude::Response> {
        let res: Result<Request,&'static str> = Request::from(req);
        Ok(iron::Response::with((iron::status::Ok, "hi")))
    }
}
pub struct Response {}
