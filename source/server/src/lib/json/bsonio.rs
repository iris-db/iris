//! Bson serialization and deserialization.

/// BSON encoding operations.
pub mod encoder {
    use std::convert::TryFrom;

    use bson::Document;
    use serde::Serialize;

    use crate::lib::json::types::JsonObject;

    /// Encodes a struct into bytes representing a BSON document.
    pub fn encode_struct<T>(s: T) -> Vec<u8>
    where
        T: Serialize,
    {
        encode_json_object(
            serde_json::to_value(s)
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        )
    }

    /// Encodes multiple structs into bytes representing a group of BSON documents.
    pub fn encode_structs<T>(s: Vec<T>) -> Vec<u8>
    where
        T: Serialize,
    {
        let acc = s
            .iter()
            .map(|i| encode_struct(i))
            .reduce(accumulate_bson_documents());

        acc.unwrap_or(Vec::new())
    }

    /// Encode a JSON object into bytes repersenting a BSON document.
    pub fn encode_json_object(object: JsonObject) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let doc = Document::try_from(object).unwrap();

        doc.to_writer(&mut buf).unwrap();

        buf
    }

    /// Encodes multiple JSON objects into bytes representing a group of BSON documents.
    pub fn encode_json_objects(objects: Vec<JsonObject>) -> Vec<u8> {
        let acc = objects
            .into_iter()
            .map(|i| encode_json_object(i))
            .reduce(accumulate_bson_documents());

        acc.unwrap_or(Vec::new())
    }

    /// Retruns a closure that accumulates all BSON documents from a document iterator.
    fn accumulate_bson_documents() -> Box<dyn Fn(Vec<u8>, Vec<u8>) -> Vec<u8>> {
        Box::new(|mut pv, mut cv| {
            cv.append(&mut pv);
            return cv;
        })
    }

    #[cfg(test)]
    mod tests {
        use serde_json::json;

        use super::*;

        #[test]
        fn test_encode_json_object() {
            let result = encode_json_object(
                json!({
                    "x": 0,
                    "y": 0
                })
                .as_object()
                .unwrap()
                .clone(),
            );
            assert!(!result.is_empty());
        }
    }
}

/// BSON decoding operations.
pub mod decoder {
    use std::io::Cursor;

    use bson::{Bson, Document};

    use crate::lib::json::types::JsonObject;
    use crate::page::error::ReadError;
    use crate::page::page::PageReadable;

    /// Decode a BSON document into a JSON struct.
    pub fn decode_document<O>(document: Vec<u8>) -> Result<O, ReadError>
    where
        O: PageReadable,
    {
        Ok(O::read(decode_json_object(document)?))
    }

    /// Decode multiple BSON documents into a vector of JSON structs.
    pub fn decode_documents<T>(documents: Vec<u8>) -> Result<Vec<T>, ReadError>
    where
        T: PageReadable,
    {
        Ok(decode_json_objects(documents)?
            .into_iter()
            .map(|o| T::read(o))
            .collect())
    }

    /// Decode a BSON document into a JSON object.
    pub fn decode_json_object(document: Vec<u8>) -> Result<JsonObject, ReadError> {
        let mut cursor = Cursor::new(document);
        let document = Document::from_reader(&mut cursor)?;

        let json = Bson::from(document).into_relaxed_extjson();
        Ok(json.as_object().unwrap().clone())
    }

    /// Decode multiple BSON documents into a vector of JSON objects.
    pub fn decode_json_objects(documents: Vec<u8>) -> Result<Vec<JsonObject>, ReadError> {
        let mut cursor = Cursor::new(&documents);
        let mut acc: Vec<Document> = Vec::new();

        loop {
            if (cursor.position() as usize) >= documents.len() - 1 {
                break;
            }

            let res = Document::from_reader(&mut cursor);

            match res {
                Ok(document) => acc.push(document),
                Err(e) => return Err(ReadError::CorruptedBsonDocument(e)),
            }
        }

        Ok(acc
            .into_iter()
            .map(|document| {
                Bson::from(document)
                    .into_relaxed_extjson()
                    .as_object()
                    .unwrap()
                    .clone()
            })
            .collect())
    }
}

#[cfg(test)]
/// Integeration tests for encoding / decoding cycles.
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::lib::json::types::JsonObject;
    use crate::page::error::ReadError;
    use crate::page::page::PageReadable;

    use super::decoder::*;
    use super::encoder::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// Test encodable and decodeable struct.
    struct Position {
        x: u64,
        y: u64,
    }

    impl PageReadable for Position {
        fn read(o: JsonObject) -> Self {
            serde_json::from_value(o.into()).unwrap()
        }
    }

    #[test]
    fn test_encode_decode_single_struct() -> Result<(), ReadError> {
        let encode_result = encode_struct(Position { x: 12, y: 24 });
        assert!(!encode_result.is_empty());

        let decode_result: Position = decode_document(encode_result)?;
        assert_eq!(decode_result, Position { x: 12, y: 24 });

        Ok(())
    }

    #[test]
    fn test_encode_decode_struct_vector() -> Result<(), ReadError> {
        let encode_result = encode_structs(vec![
            Position { x: 12, y: 24 },
            Position { x: 6, y: 12 },
            Position { x: 3, y: 6 },
        ]);
        assert!(!encode_result.is_empty());

        let decode_result = decode_documents::<Position>(encode_result)?;
        assert!(decode_result.contains(&Position { x: 12, y: 24 }));
        assert!(decode_result.contains(&Position { x: 6, y: 12 }));
        assert!(decode_result.contains(&Position { x: 3, y: 6 }));

        Ok(())
    }
}
