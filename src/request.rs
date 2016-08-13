pub struct Request {
    session: Session,
    body: RequestBody,
}
pub enum RequestBody {
    IntentRequest(IntentRequest),
    LaunchRequest(LaunchRequest),
    SessionEndedRequest(SessionEndedRequest),
}
#[derive(Debug)]
pub struct IntentRequest {
    name: String,
    slots: BTreeMap<String,String>,
}
pub struct LaunchRequest{}
pub struct SessionEndedRequest{}
impl Request {
    pub fn from() -> Result<Request,&str> {
        let json_body = req.get::<bodyparser::Raw>();
        match json_body {
            Ok(Some(json_body)) => handle_json(&json_body),
            Ok(None) => Err("No request body"),
            Err(err) => Err("Error reading request body"),
        }
    }
    fn handle_json(json: &str) -> Result<Request,&str> {
        let parsed: RequestInternal = serde_json::from_str(json).unwrap();
        parsed.into_request()
    }
}

#[derive(Deserialize)]
struct RequestInternal {
    version: String,
    session: Session,
    request: RequestBodyInternal,
}
#[derive(Deserialize)]
struct Session {
    new: bool,
    sessionId: String,
    attributes: BTreeMap<String, serde_json::Value>,
    application : Application,
    user: User,
}
#[derive(Deserialize)]
struct Application {
    applicationId: String,
}
#[derive(Deserialize)]
struct User {
    userId: String,
    accessToken: String,
}
#[derive(Deserialize)]
struct RequestBodyInternal {
    #[serde(rename="type")]
    request_type: RequestType,
    requestId: String,
    timestamp: String,
    intent: Option<IntentInternal>,
    reason: Option<Reason>,
}
#[derive(Deserialize)]
enum RequestType {
    LaunchRequest,
    IntentRequest,
    SessionEndedRequest,
}
#[derive(Deserialize)]
struct IntentInternal {
    name: String,
    slots: BTreeMap<String, SlotInternal>,
}
#[derive(Deserialize)]
struct SlotInternal {
    name: String,
    value: String,
}
#[derive(Deserialize)]
pub enum Reason {
    USER_INITIATED,
    ERROR,
    EXCEEDED_MAX_REPROMPTS,
}

impl RequestInternal {
    fn into_request(self) -> Request {
    }
}
