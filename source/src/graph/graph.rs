extern crate test;

use std::fmt::{Display, Formatter};
use std::io::Error;
use std::{fmt, fs, io};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::graph::node::{CreateNodeData, Node, NodeId};
use crate::io::page;
use crate::io::page::{PageSerializable, WriteError, MAX_PAGE_SIZE};
use crate::lib::bson::{encode, JsonObject};
use crate::lib::uid::IntCursor;
use std::fs::{File, OpenOptions};
use std::time::Instant;

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

impl Graph {
  /// Creates a new graph along and initializes a page.
  pub fn new(name: &str) -> Result<Graph, page::WriteError> {
    page::new(name)?;

    Ok(Graph {
      name: name.to_string(),
      cursor: IntCursor::new(),
      nodes: Vec::new(),
      page_pos: 0,
    })
  }

  /// Next available node id
  pub fn next_id(&mut self) -> u64 {
    self.cursor.next()
  }
}

/// The time it takes to complete an operation in milliseconds.
type OperationTime = u128;

// Crud operations.
impl Graph {
  /// Inserts a node into the graph.
  pub fn insert(&mut self, node: Node) -> Result<OperationTime, page::WriteError> {
    let time = Instant::now();

    let path = page::get_next_page_path(&*self.name);
    let mut file = OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .open(path)?;

    page::write(&mut file, node)?;

    Ok(time.elapsed().as_millis())
  }
}
