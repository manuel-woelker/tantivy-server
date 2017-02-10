
use iron::prelude::*;
use iron::Handler;

use tantivy;

use server::JsonResponse;
use bodyparser;

use service::{SearchServiceHandle, IndexDescription};

#[derive(Debug, Serialize)]
struct StatusResponse {
    health: &'static str,
    version: &'static str,
    tantivy_version: &'static str,
    number_of_indices: u64,
}

pub struct StatusHandler {
    service_handle: SearchServiceHandle,
}

impl StatusHandler {
    pub fn new(service_handle: SearchServiceHandle) -> StatusHandler {
        StatusHandler {
            service_handle: service_handle
        }
    }
}

impl Handler for StatusHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        println!("JSON body: {:?}", json_body);
        let status_response = StatusResponse {
            health: "octarine",
            version: env!("CARGO_PKG_VERSION"),
            tantivy_version: tantivy::version(),
            number_of_indices: self.service_handle.read().expect("rwlock").index_descriptions.len() as u64,
        };
        Ok(Response::with(JsonResponse(status_response)))
    }
}
