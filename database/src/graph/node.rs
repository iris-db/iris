use crate::graph::edge::Edge;

/// A unit of data. Nodes can be connect with other nodes through edges. They store data in BSON
/// format.
pub struct Node {
    pub id: NodeId,
    pub bson: String,
    pub edges: Vec<Edge>,
}

/// The primary unique identifier of a node.
pub type NodeId = u128;
