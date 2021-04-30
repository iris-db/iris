extern crate test;

use std::fmt::{Display, Formatter};
use std::io::Error;
use std::{fmt, fs};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::graph::node::{CreateNodeData, Node, NodeId};
use crate::io::page;
use crate::io::page::MAX_PAGE_SIZE;
use crate::lib::bson::{encode, Json, JsonObject};
use crate::lib::uid::IntCursor;

/// A collection of graph nodes.
pub struct Graph {
  /// The name of the graph.
  name: String,
  /// Next available id.
  cursor: IntCursor,
  /// Nodes present within the graph. Always sorted by id.
  nodes: Vec<Box<Node>>,
  /// Current page. A page is an 8KB file that contains serialized nodes. They are loaded into
  /// the nodes vec that tries to load as many nodes in memory as possible (generally the maximum
  /// amount of nodes that the nodes vec holds is < 32GB.
  page_pos: u32,
}

/// Result of a crud operation.
#[derive(Serialize, Deserialize)]
pub struct CrudOperationMetadata {
  pub count: u32,
  pub time: u32,
}

impl From<CrudOperationMetadata> for JsonObject {
  fn from(meta: CrudOperationMetadata) -> Self {
    serde_json::to_value(meta)
      .unwrap()
      .as_object()
      .unwrap()
      .clone()
  }
}

/// Error that occurs while serializing a node. Errors can be caused by the filesystem or a node
/// format error, such as exceeding the maximum node size.
pub enum SerializationError {
  /// Caused by the rust fs api.
  Filesystem(Error),
  /// Caused by a node exceeded the maximum node size.
  NodeSizeExceeded(NodeId),
}

impl From<&SerializationError> for JsonObject {
  fn from(err: &SerializationError) -> Self {
    return match err {
			SerializationError::Filesystem(e) => {
				let s = e.to_string();

				Json::from(json!({
          "error": {
            "msg": format!("CRITICAL FILESYSTEM ERROR: {}", s),
            "data": s
          }
        })).to_object()
			}
			SerializationError::NodeSizeExceeded(id) => Json::from(json!({
        "error": {
          "msg": format!("[Node Id: {}] Exceeded the maximum node size of {} bytes", id, MAX_PAGE_SIZE),
          "data": id
        }
      })).to_object()
		};
  }
}

impl Display for SerializationError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    return match self {
      SerializationError::Filesystem(e) => {
        write!(f, "[Filesystem] SerializationError: {}", e.to_string())
      }
      SerializationError::NodeSizeExceeded(id) => {
        write!(f, "[NodeSizeExceeded] SerializationError: {}", id)
      }
    };
  }
}

impl Graph {
  pub fn new(name: &str) -> Graph {
    page::new(name);

    Graph {
      name: name.to_string(),
      cursor: IntCursor::new(),
      nodes: Vec::new(),
      page_pos: 0,
    }
  }
}

// Crud operations.
impl Graph {
  /// Inserts a group of nodes into the graph.
  fn insert(&mut self, nodes: Vec<Box<Node>>) -> Result<(), SerializationError> {
    // for node in nodes {
    //
    //
    // 	page::write(node, )
    //
    //   self.nodes.push(node);
    //
    // }

    Ok(())
  }
}
