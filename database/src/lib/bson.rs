use crate::graph::node_plane::NodePlane;
use bson::{Bson, Document};
use serde_json::{Map, Value};
use std::convert::{TryFrom, TryInto};
use std::fs::File;

/// Represents a serde json object.
pub type JsonObject = Map<String, Value>;

/// Encodes a bson string into a byte vector containing the bson chunk.
pub fn encode(bson: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    let doc = Document::try_from(serde_json::from_str::<JsonObject>(bson).unwrap()).unwrap();

    doc.to_writer(&mut buf).unwrap();

    buf
}

/// Decodes all bson documents from a file and marshals them as a node.
pub fn decode_file(mut file: File, plane: &mut NodePlane) {
    let mut acc: Vec<Document> = Vec::new();

    while let Ok(deserialized) = Document::from_reader(&mut file) {
        acc.push(deserialized);
    }

    acc.iter().for_each(|d| {
        let v = Bson::from(d).into_relaxed_extjson();
        // TODO Edge insertion
        plane.insert_node(Some(v.as_str().unwrap()), None);
    });
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
