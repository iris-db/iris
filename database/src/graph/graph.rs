extern crate test;

use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::time::Instant;
use std::{fmt, fs};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::graph::node::{CreateNodeData, Node, NodeId};
use crate::io::filesystem::DATA_PATH;
use crate::lib::bson::{encode, Json, JsonObject};
use crate::lib::uid::IntCursor;

/// Default configuration.
const MAX_NODE_SIZE: usize = 8000;
const MAX_PAGE_SIZE: u64 = 16000;

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

impl Into<JsonObject> for CrudOperationMetadata {
  fn into(self) -> JsonObject {
    serde_json::to_value(self)
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

impl Into<JsonObject> for &SerializationError {
  fn into(self) -> JsonObject {
    return match self {
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
          "msg": format!("[Node Id: {}] Exceeded the maximum node size of {} bytes", id, MAX_NODE_SIZE),
          "data": id
        }
      })).to_object()
    };
  }
}

impl Display for SerializationError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
  /// Creates a new node in the graph.
  pub fn new(name: &str) -> Box<Graph> {
    Box::from(Graph {
      name: name.to_string(),
      cursor: IntCursor::new(),
      nodes: Vec::new(),
      page_pos: 0,
    })
  }

  /// Inserts a set of nodes into the graph.
  ///
  /// Returns the operation result and a vec of equal length to the nodes vec containing the
  /// node ids.
  pub fn insert_nodes(
    &mut self,
    data: Option<Vec<CreateNodeData>>,
  ) -> Result<(CrudOperationMetadata, Vec<NodeId>), SerializationError> {
    let now = Instant::now();

    let data = data.unwrap_or(Vec::new());

    let mut ids: Vec<NodeId> = Vec::new();
    let mut count = 0;

    for CreateNodeData(bson, edges) in data {
      let id = self.cursor.next();

      let n = Node::new(
        id,
        bson.unwrap_or("{}".to_string()),
        edges.unwrap_or(Vec::new()),
      );

      match self.serialize_node(&n) {
        Some(e) => return Err(e),
        None => {}
      };

      ids.push(id);

      self.nodes.push(Box::from(n));
      self.nodes.sort();

      count += 1;
    }

    Ok((
      CrudOperationMetadata {
        count,
        time: now.elapsed().as_millis() as u32,
      },
      ids,
    ))
  }

  pub fn delete_node_by_id(&mut self, id: NodeId) -> CrudOperationMetadata {
    let now = Instant::now();
    let mut count = 0;

    let pos = self.nodes.binary_search_by(|node| node.id().cmp(&id));
    let pos = match pos {
      Ok(pos) => pos,
      Err(_) => {
        return CrudOperationMetadata {
          count,
          time: now.elapsed().as_millis() as u32,
        };
      }
    };

    self.nodes.remove(pos);
    count += 1;

    CrudOperationMetadata {
      count,
      time: now.elapsed().as_millis() as u32,
    }
  }

  /// Deletes a node based on a predicate.
  pub fn delete_node_where(
    &mut self,
    predicate: fn(&Node) -> bool,
    limit: Option<u32>,
  ) -> CrudOperationMetadata {
    let mut count = 0;
    let nodes = &mut self.nodes;

    let limit_exists = limit.is_some();

    let now = Instant::now();

    for i in 0..nodes.len() {
      let n = nodes.get(i).unwrap();

      if predicate(n) {
        nodes.remove(i);
        count += 1;
      }

      if limit_exists && count == limit.unwrap() {
        break;
      }
    }

    CrudOperationMetadata {
      count,
      time: now.elapsed().as_millis() as u32,
    }
  }

  /// Gets nodes by a condition.
  pub fn get_nodes_where<P: FnOnce(&Node) -> bool + Copy>(
    &mut self,
    predicate: P,
    limit: Option<u32>,
  ) -> Vec<&Box<Node>> {
    let mut acc: Vec<&Box<Node>> = Vec::new();

    let mut count: u32 = 0;
    let limit = limit.unwrap_or(u32::MAX);

    for node in &self.nodes {
      if predicate(&node) {
        count += 1;
        acc.push(&node);
      }

      if count == limit {
        break;
      }
    }

    acc
  }
}

// Serialization API. Methods for serializing data to the disk.
impl Graph {
  /// Serializes a node onto the filesystem.
  ///
  /// Returns the serialization time if successful.
  fn serialize_node(&mut self, node: &Node) -> Option<SerializationError> {
    let mut bytes = encode(node.bson());

    let path = self.current_data_path();

    let mut open_opts = OpenOptions::new();
    let file_opts = open_opts.write(true).append(true).create(true);

    let file = file_opts.open(path.clone());
    let mut file = match file {
      Ok(v) => v,
      Err(e) => return Some(SerializationError::Filesystem(e)),
    };

    let metadata = fs::metadata(path.clone());
    let metadata = match metadata {
      Ok(v) => v,
      Err(e) => return Some(SerializationError::Filesystem(e)),
    };

    if bytes.len() > MAX_NODE_SIZE {
      return Some(SerializationError::NodeSizeExceeded(*node.id()));
    }

    if metadata.len() > MAX_PAGE_SIZE {
      self.page_pos += 1;
      file = match file_opts.open(self.current_data_path()) {
        Ok(v) => v,
        Err(e) => return Some(SerializationError::Filesystem(e)),
      }
    }

    match file.write(&mut bytes) {
      Err(e) => return Some(SerializationError::Filesystem(e)),
      _ => {}
    }

    None
  }

  /// File location of the active page.
  fn current_data_path(&self) -> String {
    format!("{}/{}.{}", DATA_PATH, self.name, self.page_pos)
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;
  use test::Bencher;

  use crate::io::filesystem;

  use super::*;

  #[test]
  fn test_insert_node() {
    filesystem::prepare();

    let mut g = Graph::new("TEST");

    let data: Vec<CreateNodeData> = vec![
      CreateNodeData(Some("{ \"hello\": \"world\" }".to_string()), None),
      CreateNodeData(Some("{ \"super\": \"cool\" }".to_string()), None),
      CreateNodeData(Some("{ \"amazing\": \"json\" }".to_string()), None),
    ];

    let res = match g.insert_nodes(Some(data)) {
      Ok(v) => v,
      Err(e) => {
        filesystem::test_utils::destroy();
        panic!("{}", e);
      }
    };

    assert!(Path::new(format!("{}/{}", DATA_PATH, "TEST.0").as_str()).exists());
    assert_eq!(res.0.count, 3);
    assert_eq!(res.1.len(), 3);

    filesystem::test_utils::destroy();
  }

  #[test]
  fn test_delete_node_by_id() {
    filesystem::prepare();

    let mut g = Graph::new("TEST");

    let data: Vec<CreateNodeData> = vec![
      CreateNodeData(None, None),
      CreateNodeData(None, None),
      CreateNodeData(None, None),
    ];

    match g.insert_nodes(Some(data)) {
      Err(e) => {
        filesystem::test_utils::destroy();
        panic!("{}", e);
      }
      _ => {}
    };

    let res_a = g.delete_node_by_id(0);
    assert_eq!(res_a.count, 1);

    let res_c = g.delete_node_by_id(2);
    assert_eq!(res_c.count, 1);

    let res_b = g.delete_node_by_id(1);
    assert_eq!(res_b.count, 1);

    let res_err = g.delete_node_by_id(4);
    assert_eq!(res_err.count, 0);
  }

  #[bench]
  fn bench_insert_nodes(b: &mut Bencher) {
    filesystem::prepare();

    let mut g = Graph::new("TEST");

    b.iter(|| {
      let data: Vec<CreateNodeData> = vec![CreateNodeData(Some("{}".to_string()), None)];

      match g.insert_nodes(Some(data)) {
        Err(e) => {
          filesystem::test_utils::destroy();
          panic!("{}", e);
        }
        _ => {}
      }
    });
  }
}
