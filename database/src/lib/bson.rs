use std::convert::TryFrom;
use std::fs::File;

use bson::{Bson, Document};
use serde_json::{Map, Value};

use crate::graph::graph::Graph;
use crate::graph::node::CreateNodeData;

/// Represents a serde json object.
pub type JsonObject = Map<String, Value>;

pub fn map_values(values: &Vec<Value>) -> Vec<&JsonObject> {
  let mut acc: Vec<&JsonObject> = Vec::new();

  values.iter().for_each(|v| acc.push(v.as_object().unwrap()));

  acc
}

/// Encodes a bson string into a byte vector containing the bson chunk.
pub fn encode(bson: &str) -> Vec<u8> {
  let mut buf: Vec<u8> = Vec::new();

  let doc = Document::try_from(serde_json::from_str::<JsonObject>(bson).unwrap()).unwrap();

  doc.to_writer(&mut buf).unwrap();

  buf
}

/// Decodes all bson documents from a file and marshals them as a node.
pub fn decode_file(mut file: File, graph: &mut Graph) {
  let mut acc: Vec<Document> = Vec::new();

  while let Ok(deserialized) = Document::from_reader(&mut file) {
    acc.push(deserialized);
  }

  let data: Vec<CreateNodeData> = acc
    .iter()
    // TODO Marshal edges.
    .map(|d| CreateNodeData(Some(Bson::from(d).into_relaxed_extjson().to_string()), None))
    .collect();

  graph.insert_nodes(Some(data));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_encodes_a_bson_string() {
    let bson = "{ \"hello\": \"world\" }";
    let bytes = encode(bson);

    assert_eq!(bytes.len() > 1, true);
  }
}
