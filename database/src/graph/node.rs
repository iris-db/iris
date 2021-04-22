use crate::graph::edge::Edge;
use std::cmp::Ordering;

/// A unit of data. Nodes can be connect with other nodes through edges. They store data in BSON
/// format.
pub struct Node {
  id: NodeId,
  bson: String,
  edges: Vec<Edge>,
}

/// Create a node from a BSON string and edges.
pub struct CreateNodeData(pub Option<String>, pub Option<Vec<Edge>>);

/// The primary unique identifier of a node.
pub type NodeId = u128;

impl Node {
  pub fn new(id: NodeId, bson: String, edges: Vec<Edge>) -> Node {
    Node { id, bson, edges }
  }

  pub fn id(&self) -> &NodeId {
    &self.id
  }

  pub fn bson(&self) -> &String {
    &self.bson
  }

  pub fn edges(&self) -> &Vec<Edge> {
    &self.edges
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    self.id.cmp(&other.id)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.id.partial_cmp(&other.id)
  }
}

impl Eq for Node {}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.id.eq(&other.id)
  }
}
