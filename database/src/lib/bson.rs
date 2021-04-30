use std::convert::TryFrom;
use std::fs::File;

use bson::{Bson, Document};
use serde_json::{Map, Value};

use crate::graph::graph::Graph;
use crate::graph::node::CreateNodeData;

/// Type alias for an unknown JSON object.
pub type JsonObject = Map<String, Value>;

/// Wrapper for easily converting to a serde JSON object.
pub struct JsonObjectWrapper(Value);

impl JsonObjectWrapper {
  /// Converts a value into a cloned JSON object.
  pub fn convert(&self) -> JsonObject {
    self.0.as_object().unwrap().clone()
  }

  /// Converts a value into a borrowed JSON object.
  pub fn convert_ref(&self) -> &JsonObject {
    self.0.as_object().unwrap()
  }
}

impl From<Value> for JsonObjectWrapper {
  fn from(v: Value) -> Self {
    JsonObjectWrapper(v)
  }
}

/// Converts a vec of Value to a vec of JsonObject.
pub fn values_to_objects(values: &Vec<Value>) -> Vec<JsonObject> {
  let mut acc: Vec<JsonObject> = Vec::new();

  values
    .iter()
    .for_each(|v| acc.push(v.as_object().unwrap().clone()));

  acc
}

/// Encodes a bson string into a byte vector containing the bson chunk.
pub fn encode(bson: JsonObject) -> Vec<u8> {
  let mut buf: Vec<u8> = Vec::new();

  let doc = Document::try_from(bson).unwrap();

  doc.to_writer(&mut buf).unwrap();

  buf
}
