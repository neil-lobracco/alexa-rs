extern crate iron;
extern crate alexa;
use iron::prelude::*;

struct RequestHandler{}
impl alexa::RequestHandler for RequestHandler {
    fn handle_request(&self, req: &alexa::Request) -> alexa::Response {
        match req.body {
            alexa::RequestBody::IntentRequest(ref ir) => {
                match ir.name.as_str() {
                    "DoubleNumber" => {
                        let num_o: Option<f64> = ir.slots.get("num").and_then(|n| n.parse().ok());
                        match num_o {
                            Some(num) => doubled_number_response(num),
                            None => i_dont_understand(),
                        }
                    },
                    _ => i_dont_understand(),
                }
            },
            _ => i_dont_understand(),
        }
    }
}
fn doubled_number_response<'a>(num: f64) -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech: Some(alexa::OutputSpeech::Text(format!("Double {} is {}",num,num * 2f64).into())),
            should_end_session: true,
        }
}
fn i_dont_understand<'a>() -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech: Some(alexa::OutputSpeech::Text("Oh no, I don't understand what you said!".into())),
            should_end_session: true,
        }
}
fn main() {
    let rh = RequestHandler{};
    let ih = alexa::IronHandler::new("Your application_id here".to_owned(),Box::new(rh));
    let chain = Chain::new(ih);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
