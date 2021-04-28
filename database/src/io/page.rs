use bson::{Bson, Document};

use crate::lib::bson::{Json, JsonObject};
use std::fs::{File, Metadata};
use std::io;
use std::io::{BufRead, Cursor, Error, Read, Write};

/// The maximum amount of data that is able to fit on a single page.
///
/// The maximum is 2MB.
pub const MAX_PAGE_SIZE: usize = 2E6 as usize;

/// Metadata about a page.
///
/// A page header contains any offsets in the page.
pub struct PageHeader {
  offsets: Vec<u8>,
}

/// Represents an object that is able to be serialized from a page.
pub trait PageSerializable {
  /// Marshall a struct into a JSON object which is eventually converted into BSON.
  fn marshall(&self) -> JsonObject;
  /// Create original struct from a JSON object.
  fn unmarshall(&self, o: JsonObject) -> Self;
}

/// Error that occurs when attempting to write data to a page.
pub enum WriteError {
  /// Io error.
  Io(io::Error),
  /// The data attempting to be written will overflow the page size.
  ///
  ///
  PageSizeExceeded(usize),
}

/// Error that occurs when attempting to read BSON documents from a page.
pub enum ReadError {
  /// Io error.
  Io(io::Error),
  CorruptedBsonDocument(bson::de::Error),
}

/// Creates a new page.
pub fn new() {}

/// Writes data to a file, restricting it to the maximum page size.
pub fn write(file: &mut File, contents: Vec<u8>) -> Result<(), WriteError> {
  let m = match file.metadata() {
    Ok(v) => v,
    Err(e) => return Err(WriteError::Io(e)),
  };

  match file.sync_all() {
    Err(e) => return Err(WriteError::Io(e)),
    _ => {}
  }

  let total_size = m.len() as usize + contents.len();

  if total_size > MAX_PAGE_SIZE {
    return Err(WriteError::PageSizeExceeded(total_size - MAX_PAGE_SIZE));
  }

  match file.write(&*contents) {
    Err(e) => return Err(WriteError::Io(e)),
    _ => {}
  }

  Ok(())
}

/// Reads a page header from a set of bytes.
///
/// A page header is a key-value string that contains metadata about the page. Header key-pairs are
/// separated by a whitespace: `KEY1=VALUE KEY2=VALUE`. Headers will always end with a newline
/// character `\n`.
///
/// Each header contains the following key-value pairs.
/// * `COUNT` - The amount of BSON documents
/// * `OFFSET` - 1 if data overflows to the next page and 0 if all data fits on the current page.
pub fn read_header() {}

/// Reads the data from a BSON page.
pub fn read_contents<S>(contents: &[u8], serializable: S) -> Result<Vec<S>, ReadError>
where
  S: PageSerializable,
{
  let mut acc: Vec<Document> = Vec::new();

  let mut cursor = Cursor::new(contents);
  // Read until the first new line to skip the page header. The page header is read separately from
  // the actual contents of the page.
  match cursor.read_until(b'\n', &mut Vec::new()) {
    Ok(s) => {
      if s == 0 {
        return Ok(Vec::new());
      }
    }
    Err(e) => return Err(ReadError::Io(e)),
  }

  loop {
    if (cursor.position() as usize) >= contents.len() - 1 {
      break;
    }

    let res = Document::from_reader(&mut cursor);

    match res {
      Ok(document) => acc.push(document),
      Err(e) => return Err(ReadError::CorruptedBsonDocument(e)),
    }
  }

  let mut fin: Vec<S> = Vec::new();

  acc
    .into_iter()
    .map(|d| Json::from(Bson::from(d).into_relaxed_extjson()).to_object())
    .for_each(|o| fin.push(serializable.unmarshall(o)));

  Ok(fin)
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::{json, Value};
  use std::convert::TryFrom;

  #[test]
  fn test_read_data() {
    let original_a = json!(
      {
        "firstName": "John",
        "lastName": "Smith"
      }
    );

    let original_b = json!(
      {
        "firstName": "Bobby",
        "lastName": "Brown"
      }
    );

    struct IdWrapper {
      first_name: String,
      last_name: String,
    }

    impl PageSerializable for IdWrapper {
      fn marshall(&self) -> JsonObject {
        json!({ "firstName": self.first_name, "lastName": self.last_name })
          .as_object()
          .unwrap()
          .clone()
      }

      fn unmarshall(&self, o: JsonObject) -> Self {
        IdWrapper {
          first_name: o.get("firstName").unwrap().as_str().unwrap().to_string(),
          last_name: o.get("lastName").unwrap().as_str().unwrap().to_string(),
        }
      }
    }

    /// Converts a JSON object into a document.
    fn to_writer(val: Value) -> Vec<u8> {
      let mut buf = Vec::new();

      Document::try_from(
        val
          .as_object()
          .expect("Could not convert to JsonObject")
          .clone(),
      )
      .expect("Could not convert to Document")
      .to_writer(&mut buf)
      .expect("Unable to write document bytes to buffer");

      buf
    }

    let buf_a = to_writer(original_a);
    let buf_b = to_writer(original_b);

    let wrapper = IdWrapper {
      first_name: "".to_string(),
      last_name: "".to_string(),
    };

    // Empty page header.
    let header = b"\n";

    let res = read_contents(&[header, &buf_a[..], &buf_b[..]].concat(), wrapper)
      .ok()
      .unwrap();

    assert_eq!(res[0].first_name, "John");
    assert_eq!(res[0].last_name, "Smith");

    assert_eq!(res[1].first_name, "Bobby");
    assert_eq!(res[1].last_name, "Brown");
  }
}
