use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Cursor, Error, Write};
use std::string::FromUtf8Error;

use bson::{Bson, Document};

use crate::io::filesystem::DATA_PATH;
use crate::lib::bson::{Json, JsonObject};
use std::path::{Path, PathBuf};

/// The maximum amount of data that is able to fit on a single page.
///
/// The standard maximum is 2MB.
pub const MAX_PAGE_SIZE: usize = 2E6 as usize;
/// File extension of the page metadata.
pub const META_PAGE_EXT: &str = "meta";

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

impl From<io::Error> for WriteError {
  fn from(e: Error) -> Self {
    WriteError::Io(e)
  }
}

/// Creates a metadata file containing the page header.
pub fn new(name: &str) -> Result<(), WriteError> {
  let mut file = File::create(get_meta_path(name))?;

  let bytes = marshall_header(Header::default());
  write(&mut file, bytes)
}

/// Opens a the next page file with available space.
pub fn open(name: &str) -> Result<File, io::Error> {
  File::open(get_next_page_path(name))
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
pub fn write<T>(to: &mut T, contents: Vec<u8>) -> Result<(), WriteError>
where
  T: Write + ComputableLength,
{
  let total_size = to.len().unwrap() + contents.len();

  if total_size > MAX_PAGE_SIZE {
    return Err(WriteError::PageSizeExceeded(total_size - MAX_PAGE_SIZE));
  }

  match to.write(&*contents) {
    Err(e) => return Err(WriteError::Io(e)),
    _ => {}
  }

  Ok(())
}

/// Gets the path of the page meta file.
pub fn get_meta_path(name: &str) -> PathBuf {
  get_page_with_ext(name, META_PAGE_EXT)
}

/// Gets the path of the page with available disk space.
pub fn get_next_page_path(name: &str) -> PathBuf {
  get_page_with_ext(name, "0")
}

/// Concatenates a file extension to a page name, returning the full relative path.
fn get_page_with_ext(name: &str, ext: &str) -> PathBuf {
  Path::new(DATA_PATH).join(&[name, ".", ext].concat())
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

/// A page header is a key-value string that contains metadata about the page. Header key-pairs are
/// separated by a whitespace: `KEY1=VALUE KEY2=VALUE`. Headers will always end with a newline
/// character `\n`.
///
/// Each header contains the following key-value pairs.
/// * `COUNT` - The amount of BSON documents
/// * `POS` - The next available page file
pub struct Header {
  /// BSON document count.
  pub count: u64,
  /// Next page with available disk space.
  pub pos: u64,
}

impl From<HashMap<String, String>> for Header {
  fn from(map: HashMap<String, String>) -> Self {
    Header {
      count: Header::get_int("COUNT", &map),
      pos: Header::get_int("POS", &map),
    }
  }
}

impl Into<HashMap<String, String>> for Header {
  fn into(self) -> HashMap<String, String> {
    vec![
      ("COUNT".to_string(), self.count.to_string()),
      ("POS".to_string(), self.pos.to_string()),
    ]
    .into_iter()
    .collect()
  }
}

impl Header {
  /// Creates a new header with default values.
  pub fn default() -> Header {
    Header { count: 0, pos: 0 }
  }

  /// Attempts to unmarshal an integer value from a key-value pair, defaulting to zero if it is not
  /// present.
  fn get_int(key: &str, map: &HashMap<String, String>) -> u64 {
    map
      .get(key)
      .unwrap_or(&"0".to_string())
      .parse()
      .unwrap_or(0)
  }
}

/// Reads a page header from a set of bytes.
pub fn read_header(contents: Vec<u8>) -> Result<Header, ReadError> {
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

  Ok(kv_pairs.into())
}

/// Marshals a page header into a byte sequence.
fn marshall_header(new: Header) -> Vec<u8> {
  let mut buf: Vec<u8> = Vec::new();

  let new: HashMap<String, String> = new.into();

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

/// Result of reading a single chunk of data from a page.
pub struct ReadObjectResult<S: PageSerializable> {
  /// The BSON object that was read.
  object: S,
  /// The position in the page where the object was read.
  ///
  /// * `0` - Start byte
  /// * `1` - End byte
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
  let mut cursor = Cursor::new(contents);
  let mut acc: Vec<(Document, (u64, u64))> = Vec::new();

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
  use crate::use_test_filesystem;

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
  fn test_new() {
    use_test_filesystem!();

    new("test").expect("Error while creating test page");

    assert!(Path::exists(&get_meta_path("test")))
  }

  #[test]
  fn test_write() {
    let mut file: Vec<u8> = Vec::new();

    match write(&mut file, b"Amazing data".to_vec()) {
      Err(e) => panic!("Something went wrong when writing: {:?}", e),
      _ => {}
    };

    assert_eq!(&*file, b"Amazing data");
  }

  #[test]
  fn test_get_meta_path() {
    let path = get_meta_path("test");
    let expected = &[DATA_PATH, "/test.meta"].concat();

    assert_eq!(path.to_str().unwrap(), expected);
  }

  #[test]
  fn test_read_header() {
    let header_raw = b"COUNT=12345 POS=1".to_vec();

    let header = read_header(header_raw)
      .ok()
      .expect("Could not read key-value pairs from header");

    assert_eq!(header.count, 12345);
    assert_eq!(header.pos, 1);
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

    let res = read_contents(&[&buf_a[..], &buf_b[..]].concat(), wrapper)
      .ok()
      .unwrap();

    let buf_a_len = buf_a.len() as u64;
    let buf_b_len = buf_b.len() as u64;

    // Object a assertions.
    let object_a = &res[0].object;

    assert_eq!(object_a.first_name, "John");
    assert_eq!(object_a.last_name, "Smith");

    let pos_b = &res[0].pos;

    assert_eq!(pos_b.0, 0);
    assert_eq!(pos_b.1, buf_a_len);

    // Object b assertions.
    let object_b = &res[1].object;

    assert_eq!(object_b.first_name, "Bobby");
    assert_eq!(object_b.last_name, "Brown");

    let pos_b = &res[1].pos;

    assert_eq!(pos_b.0, buf_a_len);
    assert_eq!(pos_b.1, buf_a_len + buf_b_len)
  }
}
