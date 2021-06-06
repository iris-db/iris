use std::convert::TryFrom;

use crate::lib::json::types::JsonObject;
use bson::Document;

/// Encodes a bson string into a byte vector containing the bson chunk.
pub fn encode(bson: JsonObject) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    let doc = Document::try_from(bson).unwrap();

    doc.to_writer(&mut buf).unwrap();

    buf
}
