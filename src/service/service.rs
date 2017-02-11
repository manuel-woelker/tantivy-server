
use std::collections::HashMap;
use std::sync::{RwLock, Arc};

use errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct IndexDescription {
    pub name: String,
}

pub struct SearchService {
    pub index_descriptions: HashMap<String, IndexDescription>,
}

impl SearchService {
    pub fn new() -> Self {
        SearchService {
            index_descriptions: HashMap::new(),
        }
    }
}

pub trait HandleFunctions {
    fn create_index(&self, request:CreateIndexRequest) -> Result<()>;
    fn get_index_descriptions(&self) -> Result<Vec<IndexDescription>>;
}

impl HandleFunctions for SearchServiceHandle {
    fn create_index(&self, request: CreateIndexRequest) -> Result<()> {
        let name = request.name.expect("name");
        self.write().expect("write").index_descriptions.insert(name.clone(), IndexDescription {name: name });
        Ok(())
    }
    fn get_index_descriptions(&self) -> Result<Vec<IndexDescription>> {
        Ok(self.read().expect("read").index_descriptions.iter().map(|(_, description)| description.clone()).collect())
    }
}

pub type SearchServiceHandle = Arc<RwLock<SearchService>>;

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
    pub name: Option<String>,
    pub mappings: HashMap<String, Mapping>,
}
