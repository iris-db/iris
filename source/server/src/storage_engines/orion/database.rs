use crate::storage_engines::orion::collection::Collection;
use std::collections::HashMap;

/// An in memory representation of the database.
pub struct Database {
    collections: HashMap<String, u32>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            collections: HashMap::new(),
        }
    }

    pub fn collections(&mut self) -> &mut HashMap<String, u32> {
        &mut self.collections
    }
}
