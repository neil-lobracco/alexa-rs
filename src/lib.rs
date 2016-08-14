extern crate iron;
extern crate chrono;
extern crate bodyparser;
extern crate serde_json;
include!(concat!(env!("OUT_DIR"), "/request.rs"));
include!(concat!(env!("OUT_DIR"), "/response.rs"));
pub trait RequestHandler: Send + Sync {
    fn handle_request(&self, &Request) -> Response;
}

pub struct IronHandler {
    application_id: String,
    request_handler: Box<RequestHandler>,
}
impl iron::middleware::Handler for IronHandler {
    fn handle(&self, req: &mut iron::Request) -> iron::prelude::IronResult<iron::prelude::Response> {
        let res: Result<Request,&'static str> = Request::from(req,&self.application_id);
        match res {
            Ok(ref req) => {
                let r = self.request_handler.handle_request(req);
                Ok(iron::Response::with((iron::status::Ok, serde_json::to_string(&r).unwrap())))
            },
            Err(s) => Ok(iron::Response::with((iron::status::BadRequest, s))),
        }

    }
}
impl IronHandler {
    pub fn new(application_id: String, request_handler: Box<RequestHandler>) -> IronHandler {
        IronHandler { application_id: application_id, request_handler: request_handler }
    }
}
