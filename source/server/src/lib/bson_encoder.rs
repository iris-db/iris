use std::convert::TryFrom;

use bson::Document;

use crate::lib::json::JsonObject;

/// Encodes a bson string into a byte vector containing the bson chunk.
pub fn encode(bson: JsonObject) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    let doc = Document::try_from(bson).unwrap();

    doc.to_writer(&mut buf).unwrap();

    buf
}
