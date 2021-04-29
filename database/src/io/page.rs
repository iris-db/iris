use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Cursor, Write};
use std::string::FromUtf8Error;

use bson::{Bson, Document};

use crate::io::filesystem::DATA_PATH;
use crate::lib::bson::{Json, JsonObject};
use std::path::{Path, PathBuf};

/// The maximum amount of data that is able to fit on a single page.
///
/// The maximum is 2MB.
pub const MAX_PAGE_SIZE: usize = 2E6 as usize;

/// Represents an object that is able to be serialized from a page.
pub trait PageSerializable {
  /// Marshall a struct into a JSON object which is eventually converted into BSON.
  fn marshall(&self) -> JsonObject;
  /// Create original struct from a JSON object.
  fn unmarshall(&self, o: JsonObject) -> Self;
}

#[derive(Debug)]
/// Error that occurs when attempting to write data to a page.
pub enum WriteError {
  /// Io error.
  Io(io::Error),
  /// The data attempting to be written will overflow the page size.
  PageSizeExceeded(usize),
}

/// Creates a metadata file containing the page header.
pub fn new(name: &str) -> Result<File, io::Error> {
  File::create(get_header_path(name))
}

/// Updates a page header with the new key-value pairs.
fn update_header(new: HashMap<String, String>) -> Vec<u8> {
  let mut buf: Vec<u8> = Vec::new();

  for (index, (key, value)) in new.iter().enumerate() {
    buf.append(&mut key.clone().into_bytes());
    buf.push(b'=');
    buf.append(&mut value.clone().into_bytes());

    if index != new.len() - 1 {
      buf.push(b' ');
    }
  }

  buf
}

/// Returns the length of the implementer as a usize.
pub trait ComputableLength {
  fn len(&self) -> Result<usize, ()>;
}

impl ComputableLength for File {
  fn len(&self) -> Result<usize, ()> {
    let m = match self.metadata() {
      Ok(m) => m,
      Err(_) => return Err(()),
    };

    Ok(m.len() as usize)
  }
}

impl<T> ComputableLength for Vec<T> {
  fn len(&self) -> Result<usize, ()> {
    Ok(Vec::len(self))
  }
}

/// Writes data to a file, restricting it to the maximum page size.
pub fn write<T>(target: &mut T, contents: &[u8]) -> Result<(), WriteError>
where
  T: Write + ComputableLength,
{
  let total_size = target.len().unwrap() + contents.len();

  if total_size > MAX_PAGE_SIZE {
    return Err(WriteError::PageSizeExceeded(total_size - MAX_PAGE_SIZE));
  }

  match target.write(&*contents) {
    Err(e) => return Err(WriteError::Io(e)),
    _ => {}
  }

  Ok(())
}

/// Returns the header path of a page by name.
pub fn get_header_path(name: &str) -> PathBuf {
  Path::new(DATA_PATH).join([name, ".head"].concat())
}

#[derive(Debug)]
/// Error that occurs when attempting to read BSON documents from a page.
pub enum ReadError {
  /// Io error.
  Io(io::Error),
  /// Error while trying to deserialize a document.
  CorruptedBsonDocument(bson::de::Error),
  /// Not a UTF8 header.
  CorruptedHeader(FromUtf8Error),
  /// Improper key value formatting.
  MalformedHeader,
}

/// Reads a page header from a set of bytes. Page headers are stored in a `<PAGE_NAME>.meta` file.
///
/// A page header is a key-value string that contains metadata about the page. Header key-pairs are
/// separated by a whitespace: `KEY1=VALUE KEY2=VALUE`. Headers will always end with a newline
/// character `\n`.
///
/// Each header contains the following key-value pairs.
/// * `COUNT` - The amount of BSON documents
/// * `OFFSET` - 1 if data overflows to the next page and 0 if all data fits on the current page.
pub fn read_header(contents: &[u8]) -> Result<HashMap<String, String>, ReadError> {
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

  let mut kv_pairs: HashMap<String, String> = HashMap::new();

  let kv_pairs_str = header.split_whitespace();
  for kv_str in kv_pairs_str {
    let kv = kv_str.split_once('=');
    let kv = match kv {
      Some(kv) => kv,
      None => return Err(ReadError::MalformedHeader),
    };

    kv_pairs.insert(kv.0.to_string(), kv.1.to_string());
  }

  Ok(kv_pairs)
}

/// Result of reading a single chunk of data from a page.
pub struct ReadObjectResult<S: PageSerializable> {
  /// The object read.
  object: S,
  /// The position in the page where it was read.
  ///
  /// * `0` - Start position
  /// * `1` - End position
  pos: (u64, u64),
}

/// Reads the data from a BSON page.
pub fn read_contents<S>(
  contents: &[u8],
  serializable: S,
) -> Result<Vec<ReadObjectResult<S>>, ReadError>
where
  S: PageSerializable,
{
  let mut acc: Vec<(Document, (u64, u64))> = Vec::new();

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

    let start_pos = cursor.position();

    let res = Document::from_reader(&mut cursor);

    let end_pos = cursor.position();

    match res {
      Ok(document) => acc.push((document, (start_pos, end_pos))),
      Err(e) => return Err(ReadError::CorruptedBsonDocument(e)),
    }
  }

  let mut fin: Vec<ReadObjectResult<S>> = Vec::new();

  acc
    .into_iter()
    .map(|read| {
      (
        Json::from(Bson::from(read.0).into_relaxed_extjson()).to_object(),
        read.1,
      )
    })
    .for_each(|res| {
      fin.push(ReadObjectResult {
        object: serializable.unmarshall(res.0),
        pos: res.1,
      })
    });

  Ok(fin)
}

#[cfg(test)]
mod tests {
  use std::convert::TryFrom;

  use serde_json::{json, Value};

  use super::*;

  /// Serializable struct for testing.
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

  #[test]
  fn test_write() {
    let mut file: Vec<u8> = Vec::new();

    match write(&mut file, b"Amazing data") {
      Err(e) => panic!("Something went wrong when writing: {:?}", e),
      _ => {}
    };

    assert_eq!(&*file, b"Amazing data");
  }

  #[test]
  fn test_read_header() {
    let header = b"COUNT=12345 OFFSET=1";

    let kvs = read_header(header)
      .ok()
      .expect("Could not read key-value pairs from header");

    assert_eq!(kvs.get("COUNT").unwrap(), "12345");
    assert_eq!(kvs.get("OFFSET").unwrap(), "1");
  }

  #[test]
  fn test_read_contents() {
    let object_a = json!(
      {
        "firstName": "John",
        "lastName": "Smith"
      }
    );

    let object_b = json!(
      {
        "firstName": "Bobby",
        "lastName": "Brown"
      }
    );

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

    let buf_a = to_writer(object_a);
    let buf_b = to_writer(object_b);

    let wrapper = IdWrapper {
      first_name: "".to_string(),
      last_name: "".to_string(),
    };

    // Empty page header.
    let header = b"\n";

    let res = read_contents(&[header, &buf_a[..], &buf_b[..]].concat(), wrapper)
      .ok()
      .unwrap();

    let header_len = header.len() as u64;
    let buf_a_len = buf_a.len() as u64;
    let buf_b_len = buf_b.len() as u64;

    // Object a assertions.
    let object_a = &res[0].object;

    assert_eq!(object_a.first_name, "John");
    assert_eq!(object_a.last_name, "Smith");

    let pos_b = &res[0].pos;

    assert_eq!(pos_b.0, header_len);
    assert_eq!(pos_b.1, header_len + buf_a_len);

    // Object b assertions.
    let object_b = &res[1].object;

    assert_eq!(object_b.first_name, "Bobby");
    assert_eq!(object_b.last_name, "Brown");

    let pos_b = &res[1].pos;

    assert_eq!(pos_b.0, header_len + buf_a_len);
    assert_eq!(pos_b.1, header_len + buf_a_len + buf_b_len)
  }
}
