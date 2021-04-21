use crate::graph::node_plane::NodePlane;
use std::sync::{RwLock, RwLockReadGuard};

/// The in memory database.
pub struct Database {
    planes: Vec<Box<NodePlane>>,
    soft_memory_limit: usize,
    hard_memory_limit: usize,
}

impl Database {
    pub fn new() -> Database {
        Database {
            soft_memory_limit: 0,
            hard_memory_limit: 0,
            planes: Database::load_planes(),
        }
    }

    pub fn planes(&self) -> &Vec<Box<NodePlane>> {
        &self.planes
    }

    pub fn planes_mut(&mut self) -> &mut Vec<Box<NodePlane>> {
        &mut self.planes
    }

    /// Load planes from disk.
    fn load_planes() -> Vec<Box<NodePlane>> {
        let mut planes = Vec::new();
        planes.push(NodePlane::new("default"));

        planes
    }
}
