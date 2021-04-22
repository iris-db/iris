use crate::aql::directive::{Directive, DirectiveList};
use crate::aql::directives::InsertDirective;
use crate::graph::node_plane::NodePlane;

/// The in memory database.
pub struct Database {
    planes: Vec<Box<NodePlane>>,
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
            planes: Database::load_planes(),
            directives: Database::register_directives(),
        }
    }

    pub fn planes(&self) -> &Vec<Box<NodePlane>> {
        &self.planes
    }

    pub fn planes_mut(&mut self) -> &mut Vec<Box<NodePlane>> {
        &mut self.planes
    }

    pub fn directives(&self) -> &DirectiveList {
        &self.directives
    }

    /// Load planes from disk.
    fn load_planes() -> Vec<Box<NodePlane>> {
        let mut planes = Vec::new();
        planes.push(NodePlane::new("default"));

        planes
    }

    /// Returns a vec of all registered directives.
    fn register_directives() -> DirectiveList {
        vec![Box::from(InsertDirective {})]
    }
}
