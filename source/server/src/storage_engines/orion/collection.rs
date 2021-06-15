use crate::storage_engines::orion::document::Document;

pub struct Collection {
    documents: Vec<Document>,
}

impl Collection {
    pub fn new() -> Self {
        Collection {
            documents: load_documents(32),
        }
    }
}

fn load_documents(max_memory: u32) -> Vec<Document> {
    Vec::new()
}
