extern crate iron;
extern crate chrono;
extern crate bodyparser;
extern crate serde_json;
include!(concat!(env!("OUT_DIR"), "/request.rs"));
pub trait RequestHandler {
    fn handle_request(&self, &Request) -> Response;
}

pub struct IronHandler {
    application_id: String,
}
impl iron::middleware::Handler for IronHandler {
    fn handle(&self, req: &mut iron::Request) -> iron::prelude::IronResult<iron::prelude::Response> {
        let res: Result<Request,&'static str> = Request::from(req,&self.application_id);
        match res {
            Ok(_) => Ok(iron::Response::with((iron::status::Ok, "hi"))),
            Err(s) => Ok(iron::Response::with((iron::status::BadRequest, s))),
        }

    }
}
impl IronHandler {
    pub fn new(application_id: String) -> IronHandler {
        IronHandler { application_id: application_id }
    }
}
pub struct Response {}
