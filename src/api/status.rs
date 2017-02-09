
use iron::prelude::*;
use iron::Handler;

use tantivy;

use server::JsonResponse;
use bodyparser;

#[derive(Debug, Serialize)]
struct StatusResponse {
    health: &'static str,
    version: &'static str,
    tantivy_version: &'static str,
}

pub struct StatusHandler {}

impl Handler for StatusHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        println!("JSON body: {:?}", json_body);
        let status_response = StatusResponse {
            health: "octarine",
            version: env!("CARGO_PKG_VERSION"),
            tantivy_version: tantivy::version(),
        };
        Ok(Response::with(JsonResponse(status_response)))
    }
}
