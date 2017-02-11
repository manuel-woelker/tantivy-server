use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use persistent::State;

use router::Router;

use bodyparser;

use service::{SearchService, IndexDescription, CreateIndexRequest, HandleFunctions};
use server::JsonResponse;
use errors::*;

pub struct CreateIndexHandler {
}

impl CreateIndexHandler {
    pub fn new() -> Self {
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
        let mut struct_body: Result<Option<CreateIndexRequest>> = req.get::<bodyparser::Struct<CreateIndexRequest>>().map_err(|e| e.into());
        let mut create_index_request = struct_body.unwrap().unwrap();
        create_index_request.name = Some(index_name);
        //        struct_body?.expect("body");
        println!("Struct body: {:?}", create_index_request);
        service_handle.create_index(create_index_request);
        Ok(Response::with(status::Ok))
    }
}


pub fn get_indices(req: &mut Request) -> IronResult<Response> {
    let service_handle = req.get::<State<SearchService>>().unwrap();
    let index_descriptions = service_handle.get_index_descriptions().unwrap();
    Ok(Response::with(JsonResponse(index_descriptions)))
}
