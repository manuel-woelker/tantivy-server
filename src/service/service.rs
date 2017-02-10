

use std::sync::{RwLock, Arc};

#[derive(Debug, Clone, Serialize)]
pub struct IndexDescription {
    pub name: String,
}

pub struct SearchService {
    pub index_descriptions: Vec<IndexDescription>,
}

impl SearchService {
    pub fn new() -> SearchService {
        SearchService {
            index_descriptions: vec![],
        }
    }
}

pub type SearchServiceHandle = Arc<RwLock<SearchService>>;