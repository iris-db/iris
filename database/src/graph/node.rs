use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::graph::edge::Edge;
use crate::io::page::PageSerializable;
use crate::lib::bson::JsonObject;
use serde_json::Value;

/// Create a node from a BSON string and edges.
pub struct CreateNodeData(pub Option<Value>, pub Option<Vec<Edge>>);

/// The primary unique identifier of a node.
pub type NodeId = u64;

/// A unit of data. Nodes can be connect with other nodes through edges. They store data in BSON
/// format.
#[derive(Serialize, Deserialize)]
pub struct Node {
  id: NodeId,
  data: Value,
  edges: Vec<Edge>,
}

impl Node {
  pub fn new(id: NodeId, data: Value, edges: Vec<Edge>) -> Node {
    Node { id, data, edges }
  }

  pub fn id(&self) -> &NodeId {
    &self.id
  }

  pub fn data(&self) -> &Value {
    &self.data
  }

  pub fn edges(&self) -> &Vec<Edge> {
    &self.edges
  }
}

impl PageSerializable for Node {
  fn marshall(&self) -> JsonObject {
    serde_json::to_value(self)
      .unwrap()
      .as_object()
      .unwrap()
      .clone()
  }

  fn unmarshall(&self, o: JsonObject) -> Self {
    serde_json::from_value(o.into()).unwrap()
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

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  pub fn test_serialization() {
    let node = Node::new(0, json!({ "hello": "world" }), Vec::new());

    let node_obj = node.marshall();

    assert_eq!(node_obj.get("id").unwrap(), 0);
    assert_eq!(node_obj.get("data").unwrap(), &json!({ "hello": "world" }));
    assert_eq!(node_obj.get("edges").unwrap(), &Value::Array(Vec::new()));
  }
}
