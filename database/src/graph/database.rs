use crate::aql::directive::{Directive, DirectiveList};
use crate::aql::directives::InsertDirective;
use crate::graph::graph::Graph;

/// The in memory database.
pub struct Database {
  graphs: Vec<Box<Graph>>,
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

  pub fn graphs(&self) -> &Vec<Box<Graph>> {
    &self.graphs
  }

  pub fn graphs_mut(&mut self) -> &mut Vec<Box<Graph>> {
    &mut self.graphs
  }

  pub fn directives(&self) -> &DirectiveList {
    &self.directives
  }

  /// Load data stores from disk.
  fn load_graphs() -> Vec<Box<Graph>> {
    let mut graphs = Vec::new();
    graphs.push(Graph::new("default"));

    graphs
  }

  /// Returns a vec of all registered directives.
  fn register_directives() -> DirectiveList {
    vec![Box::from(InsertDirective {})]
  }
}
