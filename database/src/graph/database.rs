use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::aql::directive::{Directive, DirectiveList};
use crate::aql::directives::{DeleteDirective, InsertDirective, ReadDirective};
use crate::graph::graph::Graph;

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
      directives: Database::register_directives()
        .into_iter()
        .map(|d| (d.key().to_string(), d))
        .collect(),
    }
  }

  pub fn route_ctx(&mut self) -> (&mut Graphs, &mut DirectiveList) {
    (&mut self.graphs, &mut self.directives)
  }

  pub fn soft_memory_limit(&self) -> &usize {
    &self.soft_memory_limit
  }

  pub fn hard_memory_limit(&self) -> &usize {
    &self.hard_memory_limit
  }

  /// Load data stores from disk.
  fn load_graphs() -> Graphs {
    HashMap::from_iter(IntoIter::new([(
      "default".to_string(),
      Graph::new("default"),
    )]))
  }

  /// Returns a vec of all registered directives.
  fn register_directives() -> Vec<&'static dyn Directive> {
    vec![&InsertDirective, &DeleteDirective, &ReadDirective]
  }
}
