use crate::graph::node::Node;
use crate::lib::uid::IntCursor;

/// A collection of Nodes.
pub struct NodePlane {
    name: String,
    nodes: Vec<Node>,
    cursor: IntCursor,
}

impl NodePlane {
    fn new(name: &str) -> NodePlane {
        NodePlane {
            name: name.to_string(),
            nodes: Vec::new(),
            cursor: IntCursor::new(),
        }
    }
}
