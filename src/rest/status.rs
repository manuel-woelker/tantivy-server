
use iron::prelude::*;
use iron::Handler;
use persistent::State;

use tantivy;

use server::JsonResponse;
use bodyparser;

use service::{SearchService, HandleFunctions};

#[derive(Debug, Serialize)]
struct StatusResponse {
    health: &'static str,
    version: &'static str,
    tantivy_version: &'static str,
    number_of_indices: u64,
}

pub struct StatusHandler {
}

impl StatusHandler {
    pub fn new() -> StatusHandler {
        StatusHandler {}
    }
}

impl Handler for StatusHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let service_handle = req.get::<State<SearchService>>().unwrap();
        let json_body = req.get::<bodyparser::Json>();
        println!("JSON body: {:?}", json_body);
        let status_response = StatusResponse {
            health: "octarine",
            version: env!("CARGO_PKG_VERSION"),
            tantivy_version: tantivy::version(),
            number_of_indices: service_handle.get_index_descriptions().unwrap().len() as u64,
        };
        Ok(Response::with(JsonResponse(status_response)))
    }
}
