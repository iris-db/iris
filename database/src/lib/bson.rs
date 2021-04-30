use std::convert::TryFrom;
use std::fs::File;

use bson::{Bson, Document};
use serde_json::{Map, Value};

use crate::graph::graph::Graph;
use crate::graph::node::CreateNodeData;

/// Type alias for an unknown JSON object.
pub type JsonObject = Map<String, Value>;

pub struct Json(Value);

impl Json {
  pub fn to_object(&self) -> JsonObject {
    self.0.as_object().unwrap().clone()
  }

  pub fn to_object_ref(&self) -> &JsonObject {
    self.0.as_object().unwrap()
  }
}

impl From<Value> for Json {
  fn from(v: Value) -> Self {
    Json(v)
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
