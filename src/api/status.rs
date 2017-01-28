
use iron::prelude::*;
use iron::Handler;

use tantivy;

use server::JsonResponse;

#[derive(Debug, Serialize)]
struct StatusResponse {
    health: &'static str,
    version: &'static str,
    tantivy_version: &'static str,
}

pub struct StatusHandler {}

impl Handler for StatusHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let status_response = StatusResponse {
            health: "octarine",
            version: env!("CARGO_PKG_VERSION"),
            tantivy_version: tantivy::version(),
        };
        Ok(Response::with(JsonResponse(status_response)))
    }
}
