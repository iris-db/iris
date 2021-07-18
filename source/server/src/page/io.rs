use std::fs::File;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

use bson::{Bson, Document};

use crate::io::filesystem::DatabasePath;
use crate::page::error::{ReadError, WriteError};
use crate::page::metadata::PageMetadata;
use crate::page::page::{PageReadable, PageWriteable, MAX_PAGE_SIZE, META_PAGE_EXT};

/// Creates a metadata file containing the page metadata and an empty file representing the page.
pub fn new(name: &str) -> Result<(), WriteError> {
    let p = &get_meta_path(name);
    fs::create_dir_all(p.parent().unwrap())?;

    let mut _page_file = File::create(get_next_page_path(name))?;
    let mut meta_file = File::create(p)?;

    let bytes: Vec<u8> = PageMetadata::new().as_bytes();
    write(&mut meta_file, bytes)
}

/// Opens a the next page file with available space.
pub fn open(name: &str) -> Result<File, io::Error> {
    File::open(get_next_page_path(name))
}

/// Returns the length of the implementer as a usize.
pub trait ComputeByteSize {
    fn len(&self) -> Result<usize, ()>;
}

impl ComputeByteSize for File {
    fn len(&self) -> Result<usize, ()> {
        let m = match self.metadata() {
            Ok(m) => m,
            Err(_) => return Err(()),
        };

        Ok(m.len() as usize)
    }
}

impl<T> ComputeByteSize for Vec<T> {
    fn len(&self) -> Result<usize, ()> {
        Ok(Vec::len(self))
    }
}

/// Writes data to a file, restricting it to the maximum page size.
pub fn write<T, S>(to: &mut T, contents: S) -> Result<(), WriteError>
where
    T: Write + ComputeByteSize,
    S: PageWriteable,
{
    let contents = contents.write();

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
    Path::new(&DatabasePath::Data.path()).join(&[name, ".", ext].concat())
}

impl PageWriteable for Vec<u8> {
    fn write(self) -> Vec<u8> {
        self
    }
}

/// Result of reading a single chunk of data from a page.
pub struct ReadObjectResult<O>
where
    O: PageReadable,
{
    /// The BSON object that was read.
    object: O,
    /// The position in the page where the object was read.
    ///
    /// * `0` - Start byte
    /// * `1` - End byte
    pos: (u64, u64),
}

/// Reads the data from a BSON page.
pub fn read_contents<O>(contents: Vec<u8>) -> Result<Vec<ReadObjectResult<O>>, ReadError>
where
    O: PageReadable,
{
    let contents = &contents;

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

    let mut fin: Vec<ReadObjectResult<O>> = Vec::new();

    acc.into_iter()
        .map(|read| {
            (
                Bson::from(read.0)
                    .into_relaxed_extjson()
                    .as_object()
                    .unwrap()
                    .clone(),
                read.1,
            )
        })
        .for_each(|res| {
            fin.push(ReadObjectResult {
                object: O::read(res.0),
                pos: res.1,
            })
        });

    Ok(fin)
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use serde_json::{json, Value};

    use crate::lib::json::types::JsonObject;

    use super::*;

    /// Serializable struct for testing.
    struct IdWrapper {
        first_name: String,
        last_name: String,
    }

    impl PageReadable for IdWrapper {
        fn read(o: JsonObject) -> Self {
            IdWrapper {
                first_name: o.get("firstName").unwrap().as_str().unwrap().to_string(),
                last_name: o.get("lastName").unwrap().as_str().unwrap().to_string(),
            }
        }
    }

    #[test]
    fn test_new() {
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
        let expected = format!("{}/{}", DatabasePath::Data.path(), "test.meta");

        assert_eq!(path.to_str().unwrap(), expected);
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
                val.as_object()
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

        let res = read_contents::<IdWrapper>([&buf_a[..], &buf_b[..]].concat())
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
