use bson::{doc, Bson, Document};
use serde_json::{Map, Value};
use std::convert::{TryFrom, TryInto};
use std::fs::File;

/// Encodes a bson string into a byte vector containing the bson chunk.
pub fn encode_bson(bson: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    let value = "{ hello: \"world\" }";
    let bson = Bson::from(value);
    let doc = bson.as_document().unwrap();

    //
    // doc.to_writer(&mut buf).unwrap();

    buf
}

/// Decodes a bson chunk from a file into bson strings.
pub fn decode_chunks(mut file: File) -> Vec<Document> {
    let mut acc: Vec<Document> = Vec::new();

    while let Ok(deserialized) = Document::from_reader(&mut file) {
        acc.push(deserialized);
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_a_bson_string() {
        let bson = "{ hello: \"world\" }";

        let bytes = encode_bson(bson);
    }
}
