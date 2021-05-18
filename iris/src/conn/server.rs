use crate::database::graph::Graph;
use crate::iql::keyword::Keyword;
use std::collections::HashMap;

/// An IrisDB server.
pub struct Server {
	max_cache_size: usize,
	port: usize,
}

/// An in memory representation of the database.
pub struct Database {
	graphs: HashMap<String, Graph>,
}

impl Database {
	/// Calculates the total amount of memory being used by the internal database.
	fn mem_size() -> u32 {
		0
	}

	fn graphs(&mut self) -> &mut HashMap<String, Graph> {
		&mut self.graphs
	}
}
