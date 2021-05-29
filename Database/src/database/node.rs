use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::database::edge::Edge;
use crate::io::page::PageSerializable;
use crate::lib::bson_encoder::encode;
use crate::lib::json::JsonObject;
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
	group: String,
	data: Value,
	edges: Vec<Edge>,
}

impl Node {
	pub fn new(
		id: NodeId,
		group: Option<String>,
		data: Option<Value>,
		edges: Option<Vec<Edge>>,
	) -> Node {
		Node {
			id,
			group: group.unwrap_or("".to_string()),
			data: data.unwrap_or(Value::Object(JsonObject::new())),
			edges: edges.unwrap_or(Vec::new()),
		}
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
	fn marshall(&self) -> Vec<u8> {
		encode(
			serde_json::to_value(self)
				.unwrap()
				.as_object()
				.unwrap()
				.clone(),
		)
	}

	fn unmarshall(o: JsonObject) -> Self {
		serde_json::from_value(Value::from(o)).unwrap()
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
