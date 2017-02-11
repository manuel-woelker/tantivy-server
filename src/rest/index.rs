use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use persistent::State;

use router::Router;

use bodyparser;

use service::{SearchService, IndexDescription};

use errors::*;

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

pub struct CreateIndexHandler {
}

impl CreateIndexHandler {
    pub fn new() -> CreateIndexHandler {
        CreateIndexHandler {}
    }
}

impl Handler for CreateIndexHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let service_handle = req.get::<State<SearchService>>().unwrap();
        let index_name = req.extensions.get::<Router>().unwrap().find("index_name").expect("index_name").to_string();
        let json_body = req.get::<bodyparser::Json>();
        println!("INDEX: {}", index_name);
        println!("JSON body: {:?}", json_body);
        let struct_body: Result<Option<CreateIndexRequest>> = req.get::<bodyparser::Struct<CreateIndexRequest>>().map_err(|e| e.into());
        //        struct_body?.expect("body");
        println!("Struct body: {:?}", struct_body);
        service_handle.write().expect("rwlock").index_descriptions.push(IndexDescription { name: index_name });
        Ok(Response::with(status::Ok))
    }
}
