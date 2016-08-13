extern crate iron;
extern crate bodyparser;
extern crate serde_json;

pub trait RequestHandler {
    fn handle_request(&self, &Request) -> Response;
}

struct IronHandler {
}
impl iron::middleware::Handler for IronHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        Result<Request> = Request::from(req);
    }
}
