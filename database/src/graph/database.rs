use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::aql::directive::DirectiveList;
use crate::aql::directives::InsertDirective;
use crate::graph::graph::Graph;
use std::sync::Arc;

pub type Graphs = HashMap<String, Box<Graph>>;

/// The in memory database.
pub struct Database {
  graphs: Graphs,
  directives: DirectiveList,
  soft_memory_limit: usize,
  hard_memory_limit: usize,
}

impl Database {
  /// Initializes a new database.
  pub fn new() -> Database {
    Database {
      soft_memory_limit: 0,
      hard_memory_limit: 0,
      graphs: Database::load_graphs(),
      directives: Database::register_directives(),
    }
  }

  pub fn ctx_data(&mut self) -> (&mut Graphs, &mut DirectiveList) {
    (&mut self.graphs, &mut self.directives)
  }

  pub fn directives_mut(&mut self) -> &mut DirectiveList {
    &mut self.directives
  }

  /// Load data stores from disk.
  fn load_graphs() -> Graphs {
    HashMap::from_iter(IntoIter::new([(
      "default".to_string(),
      Graph::new("default"),
    )]))
  }

  /// Returns a vec of all registered directives.
  fn register_directives() -> DirectiveList {
    vec![Arc::new(InsertDirective)]
  }
}
