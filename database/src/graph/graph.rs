extern crate test;

use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::time::Instant;
use std::{fmt, fs};

use serde_json::{json, Value};

use crate::graph::node::{CreateNodeData, Node, NodeId};
use crate::lib::bson::encode;
use crate::lib::filesystem::DATA_PATH;
use crate::lib::uid::IntCursor;

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
pub struct CrudOperationResult {
  count: u32,
  time: u128,
}

/// Error that occurs while serializing a node. Errors can be caused by the filesystem or a node
/// format error, such as exceeding the maximum node size.
pub enum SerializationError {
  /// Caused by the rust fs api.
  Filesystem(Error),
  /// Caused by a node exceeded the maximum node size.
  NodeSizeExceeded(NodeId),
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

/// Methods for marshaling errors into JSON error objects.
impl SerializationError {
  fn marshal_filesystem_error(e: Error) -> Value {
    let s = e.to_string();

    json!({
        "error": {
            "msg": format!("CRITICAL FILESYSTEM ERROR: {}", s),
            "data": s
        }
    })
  }

  fn marshal_node_size_exceeded_error(id: NodeId) -> Value {
    json!({
        "error": {
            "msg": format!("[Node Id: {}] Exceeded the maximum node size of {} bytes", id, MAX_NODE_SIZE),
            "data": id
        }
    })
  }
}

/// Getters.
impl Graph {
  pub fn name(&self) -> &String {
    &self.name
  }
}

/// Public API. This includes CRUD operations.
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
  ) -> Result<(CrudOperationResult, Vec<NodeId>), SerializationError> {
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
      CrudOperationResult {
        count,
        time: now.elapsed().as_millis(),
      },
      ids,
    ))
  }

  pub fn delete_node_by_id(&mut self, id: NodeId) -> CrudOperationResult {
    let now = Instant::now();
    let mut count = 0;

    let pos = self.nodes.binary_search_by(|node| node.id().cmp(&id));
    let pos = match pos {
      Ok(pos) => pos,
      Err(_) => {
        return CrudOperationResult {
          count,
          time: now.elapsed().as_millis(),
        };
      }
    };

    self.nodes.remove(pos);
    count += 1;

    CrudOperationResult {
      count,
      time: now.elapsed().as_millis(),
    }
  }

  /// Deletes a node based on a predicate.
  pub fn delete_node_where(
    &mut self,
    predicate: fn(&Node) -> bool,
    limit: Option<u32>,
  ) -> CrudOperationResult {
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

    CrudOperationResult {
      count,
      time: now.elapsed().as_millis(),
    }
  }
}

/// Serialization API. Methods for serializing data to the disk.
impl Graph {
  /// Serializes a node onto the filesystem.
  ///
  /// Returns the serialization time if successful.
  fn serialize_node(&mut self, node: &Node) -> Option<SerializationError> {
    let mut bytes = encode(node.bson());

    let path = self.current_data_path();

    let mut open_opts = OpenOptions::new();
    let file_opts = open_opts.write(true).create(true);

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
  use test::Bencher;

  use super::*;
  use std::path::Path;

  #[test]
  fn test_insert_node() {
    let mut p = Graph::new("TEST");

    let mut data: Vec<CreateNodeData> = Vec::new();
    data.push(CreateNodeData(
      Some("{ \"hello\": \"world\" }".to_string()),
      None,
    ));
    data.push(CreateNodeData(
      Some("{ \"hello\": \"world\" }".to_string()),
      None,
    ));
    data.push(CreateNodeData(
      Some("{ \"hello\": \"world\" }".to_string()),
      None,
    ));

    let res = match p.insert_nodes(Some(data)) {
      Ok(v) => v,
      Err(e) => panic!("{}", e),
    };

    assert!(Path::new(format!("{}/{}", DATA_PATH, "TEST.0").as_str()).exists());
    assert_eq!(res.0.count, 3);
    assert_eq!(res.1.len(), 3);
  }

  #[test]
  fn test_delete_node_by_id() {
    let mut p = Graph::new("TEST");

    let mut data: Vec<CreateNodeData> = Vec::new();
    // 3 empty nodes.
    data.push(CreateNodeData(None, None));
    data.push(CreateNodeData(None, None));
    data.push(CreateNodeData(None, None));

    match p.insert_nodes(Some(data)) {
      Err(e) => panic!("{}", e),
      _ => {}
    };

    let res_a = p.delete_node_by_id(0);
    assert_eq!(res_a.count, 1);

    let res_c = p.delete_node_by_id(2);
    assert_eq!(res_c.count, 1);

    let res_b = p.delete_node_by_id(1);
    assert_eq!(res_b.count, 1);

    let res_err = p.delete_node_by_id(4);
    assert_eq!(res_err.count, 0);
  }

  #[bench]
  fn bench_insert_nodes(b: &mut Bencher) {
    let mut p = Graph::new("TEST");

    b.iter(|| {
      let mut data: Vec<CreateNodeData> = Vec::new();
      data.push(CreateNodeData(Some("{}".to_string()), None));

      match p.insert_nodes(Some(data)) {
        Err(e) => panic!("{}", e),
        _ => {}
      }
    });
  }
}