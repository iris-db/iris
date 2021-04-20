use crate::graph::node_plane::NodePlane;

/// The in memory database.
pub struct Database {
    soft_memory_limit: usize,
    hard_memory_limit: usize,
    planes: Vec<Box<NodePlane>>,
}

impl Database {
    pub fn new() -> Box<Database> {
        Box::from(Database {
            soft_memory_limit: 0,
            hard_memory_limit: 0,
            planes: Database::load_planes(),
        })
    }

    /// Load planes from disk.
    fn load_planes() -> Vec<Box<NodePlane>> {
        Vec::new()
    }
}
