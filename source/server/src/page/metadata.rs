use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{BufRead, Cursor};

use crate::page::error::ReadError;

/// A page metadata file is a key-value string that contains metadata about the page. Header
/// key-pairs are separated by a whitespace: `KEY1=VALUE KEY2=VALUE`.
///
/// Each header contains the following key-value pairs.
/// * `COUNT` - The amount of BSON documents
/// * `POS` - The next available page file
pub struct PageMetadata {
    /// BSON document count.
    pub count: u64,
    /// Next page with available disk space.
    pub pos: u64,
}

impl TryFrom<Vec<u8>> for PageMetadata {
    type Error = ReadError;

    fn try_from(contents: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(contents);
        let mut header_bytes = Vec::new();

        match cursor.read_until(b'\n', &mut header_bytes) {
            Err(e) => return Err(ReadError::Io(e)),
            _ => {}
        }

        let header = String::from_utf8(header_bytes);
        let header = match header {
            Ok(h) => h,
            Err(e) => return Err(ReadError::CorruptedHeader(e)),
        };

        let mut kv_pairs: HashMap<String, _> = HashMap::new();

        let kv_pairs_str = header.split_whitespace();
        for kv_str in kv_pairs_str {
            let kv = kv_str.split_once('=');
            let kv = match kv {
                Some(kv) => kv,
                None => return Err(ReadError::MalformedHeader),
            };

            kv_pairs.insert(kv.0.to_string(), kv.1);
        }

        Ok(PageMetadata { count: 0, pos: 0 })
    }
}

impl PageMetadata {
    /// Initializes a new PageMetadata object, not saving it to the disk.
    pub fn new() -> Self {
        PageMetadata { count: 0, pos: 0 }
    }

    /// The file name where the metadata should be written to.
    pub fn file_name(&self, collection_name: String) -> String {
        format!("{}.{}", collection_name, self.pos)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!("COUNT={}\nPOS={}", self.count, self.pos).into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name() {
        let mut metadata = PageMetadata::new();
        metadata.pos = 64;

        assert_eq!(metadata.file_name("users".into()), "users.64");
    }

    #[test]
    fn test_as_bytes() {
        let mut metadata = PageMetadata::new();
        metadata.count = 32;
        metadata.pos = 64;

        let expected = b"COUNT=32\nPOS=64";

        assert_eq!(expected, metadata.as_bytes());
    }
}
