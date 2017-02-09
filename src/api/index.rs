use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;

use bodyparser;

#[derive(Debug, Clone, Deserialize)]
pub struct FieldDescriptor {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mapping {
    properties: HashMap<String, FieldDescriptor>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateIndexRequest {
    mappings: HashMap<String, Mapping>,
}

pub struct CreateIndexHandler {}

impl Handler for CreateIndexHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        println!("JSON body: {:?}", json_body);
        let struct_body = req.get::<bodyparser::Struct<CreateIndexRequest>>();
        println!("Struct body: {:?}", struct_body);
        Ok(Response::with(status::Ok))
    }
}
