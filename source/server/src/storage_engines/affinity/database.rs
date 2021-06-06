use crate::storage_engines::affinity::graph::Graph;
use std::collections::HashMap;

/// An in memory representation of the database.
pub struct Database {
    graphs: HashMap<String, Graph>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            graphs: HashMap::new(),
        }
    }

    pub fn graphs(&mut self) -> &mut HashMap<String, Graph> {
        &mut self.graphs
    }
}
