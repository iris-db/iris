use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::graph::graph::Graph;
use crate::query::directive::{Directive, DirectiveList};

pub type Graphs = HashMap<String, Box<Graph>>;

/// The in memory database representation.
pub struct Database {
  graphs: Graphs,
  directives: DirectiveList,
}

impl Database {
  /// Initializes a new database.
  pub fn new() -> Database {
    Database {
      graphs: Database::load_graphs(),
      directives: Database::register_directives()
        .into_iter()
        .map(|d| (d.key().to_string(), d))
        .collect(),
    }
  }

  /// Returns the http route context.
  pub fn route_ctx(&mut self) -> (&mut Graphs, &mut DirectiveList) {
    (&mut self.graphs, &mut self.directives)
  }

  /// Load data stores from disk.
  fn load_graphs() -> Graphs {
    HashMap::from_iter(IntoIter::new([(
      "default".to_string(),
      Box::new(Graph::new("default")),
    )]))
  }

  /// Returns a vec of all registered directives.
  fn register_directives() -> Vec<&'static dyn Directive> {
    vec![]
  }
}
