use crate::lib::bson::{Json, JsonObject};
use bson::{Bson, Document};
use std::fs::File;
use std::io;
use std::path::Path;

/// Maximum amount of data that is able to fit on a single page.
///
/// The maximum is 2MB.
pub const MAX_PAGE_SIZE: u32 = 2E6 as u32;

/// Pages are a portion of the disk data containing node data. They are restricted by a maximum
/// size. When the size is exceeded there will be a page offset and the data will leak onto the next
/// page.
pub struct Page {
  /// The underlying page file.
  file: File,
}

/// Metadata about a page.
///
/// A page header contains any offsets in the page.
pub struct PageHeader {
  offsets: Vec<u8>,
}

/// Represents an object that is able to be serialized from a page.
trait PageSerializable {
  /// Marshall a struct into a JSON object which is eventually converted into BSON.
  fn marshall(&self) -> JsonObject;
  /// Create original struct from a JSON object.
  fn unmarshall(&self, o: JsonObject) -> Self;
}

impl Page {
  /// Attempts to create a new page from the relative path.
  pub fn try_new(path: &str) -> Result<Page, io::Error> {
    let file = File::open(Path::new(path))?;

    Ok(Page { file })
  }

  /// Reads the page header.
  pub fn read_header() {}

  pub fn read<S>(&mut self, serializable: S) -> Vec<S>
  where
    S: PageSerializable,
  {
    let mut acc = Vec::new();

    while let Ok(deserialized) = Document::from_reader(&mut self.file) {
      acc.push(deserialized);
    }

    let mut fin: Vec<S> = Vec::new();

    acc
      .into_iter()
      .map(|d| Json::from(Bson::from(d).into_relaxed_extjson()).to_object())
      .for_each(|o| fin.push(serializable.unmarshall(o)));

    fin
  }
}

#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn test_read() {
    let mut page = Page::try_new("").unwrap();

    struct IdWrapper {
      id: i32,
    };

    impl PageSerializable for IdWrapper {
      fn marshall(&self) -> JsonObject {
        todo!()
      }

      fn unmarshall(&self, o: JsonObject) -> Self {
        todo!()
      }
    }

    let serializable = IdWrapper { id: 0 };

    let wrappers = page.read(serializable);
  }
}
