use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::graph::graph::Graph;

/// The in memory iris representation.
pub struct Database {
	graphs: HashMap<String, Box<Graph>>,
}

impl Database {
	/// Initializes a new iris.
	pub fn new() -> Database {
		Database {
			graphs: Database::load_graphs(),
		}
	}

	pub fn graphs(&mut self) -> &mut HashMap<String, Box<Graph>> {
		&mut self.graphs
	}

	/// Load data stores from disk.
	fn load_graphs() -> HashMap<String, Box<Graph>> {
		HashMap::from_iter(IntoIter::new([(
			"default".to_string(),
			Box::new(Graph::new("default").unwrap()),
		)]))
	}
}
