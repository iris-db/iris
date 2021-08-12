use std::fs;

use crate::io::path::DatabasePath;
use crate::lib::json::types::JsonObject;
use crate::page::error::{ReadError, WriteError};
use crate::page::metadata::PageMetadata;
use crate::storage::document::Document;
use crate::storage::utils::CollectionNameFormatter;

/// The maximum amount of data that is able to fit on a single page.
///
/// The standard maximum is 2MB.
pub const MAX_PAGE_SIZE: usize = 2E6 as usize;
/// File extension of the page metadata.
pub const META_PAGE_EXT: &str = "meta";

pub trait PageReadable {
    fn read(o: JsonObject) -> Self;
}

pub trait PageWriteable {
    fn write(self) -> Vec<u8>;
}

/// A single chunk of data from a database collection. Can be loaded in and out from memory when
/// necessary.
pub struct Page {
    collection_name: CollectionNameFormatter,
    /// The page id for the collection.
    metadata: PageMetadata,
    /// Document data possibly loaded in memory. If the documents is None, then the page is not
    /// loaded into memory.
    documents: Option<Box<Vec<Document>>>,
}

impl Page {
    /// Creates a new page on the filesystem if it does not exist.
    pub fn create(collection_name: String) -> Self {
        todo!()
    }

    /// Loads the page contents into memory.
    pub fn read(&mut self) -> Result<(), ReadError> {
        todo!()
    }

    /// Frees the page contents from memory.
    pub fn free(&mut self) {
        let contents = &mut self.documents;
        if contents.is_some() {
            let contents = contents.as_mut().unwrap();
            contents.clear();
        }
    }

    /// Get the page contents from memory if loaded in memory.
    pub fn data(&self) -> &Option<Box<Vec<Document>>> {
        &self.documents
    }

    /// Updates the page contents. If the page is loaded into memory, the contents are updated in
    /// memory as well as on the filesystem, otherwise only updating on the filesystem.
    pub fn write(&mut self, new: Vec<Document>) {
        todo!()
    }

    /// Write the page metadata to the filesystem.
    pub fn fwrite_meta(&self) -> Result<(), WriteError> {
        let file_name = self
            .metadata
            .file_name(self.collection_name.clone().into_original());
        let bytes = self.metadata.as_bytes();

        let err = fs::write(DatabasePath::Data.file(&file_name), bytes).err();
        return match err {
            None => Ok(()),
            Some(e) => Err(e.into()),
        };
    }
}
