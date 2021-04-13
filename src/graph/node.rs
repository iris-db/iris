use crate::graph::node_plane::NodePlane;
use crate::io::disk::{DeserializationResult, Serializable, SerializationResult};
use crate::lib::common::JsonObject;

pub struct Node {
    id: u128,
    data: Option<JsonObject>,
}

/// Crud operations for Nodes.
impl Node {
    pub fn new(plane: &NodePlane, data: Option<JsonObject>) -> Node {
        Node {
            id: generate_uid(plane),
            data,
        }
    }

    pub fn delete(&self) {}
}

impl Serializable<JsonObject> for Node {
    fn write(&self) -> SerializationResult {
        todo!()
    }

    fn read(&self) -> DeserializationResult<JsonObject> {
        todo!()
    }

    fn get_data_path(&self) -> String {
        todo!()
    }
}

/// Generates a uid for a Node.
fn generate_uid(plane: &NodePlane) -> u128 {
    2
}
