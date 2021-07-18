use std::fs;

use crate::io::filesystem::DatabasePath;
use crate::lib::json::types::JsonObject;
use crate::page::error::{ReadError, WriteError};
use crate::page::metadata::PageMetadata;
use crate::storage_engines::orion::collection::CollectionName;
use crate::storage_engines::orion::document::Document;

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

/// A set of data limited to the max page size.
pub struct Page {
    /// Filesystem path relative to the database data path.
    collection_name: CollectionName,
    /// The page id for the collection.
    metadata: PageMetadata,
    /// Page data loaded in memory.
    documents: Option<Box<Vec<Document>>>,
}

impl Page {
    /// Creates a new page on the filesystem if it does not exist.
    pub fn create(collection_name: CollectionName) -> Self {
        let mut page = Page {
            collection_name,
            metadata: PageMetadata::new(),
            documents: None,
        };

        page.fwrite_all().unwrap();

        page
    }

    /// Loads the page contents into memory.
    pub fn load_contents_into_memory(&mut self) -> Result<(), ReadError> {
        todo!()
    }

    /// Frees the page contents from memory.
    pub fn release_contents_from_memory(&mut self) {
        let contents = &mut self.documents;
        if contents.is_some() {
            let contents = contents.as_mut().unwrap();
            contents.clear();
        }
    }

    /// Get the page contents from memory if loaded in memory, otherwise getting them directly
    /// by opening the file.
    pub fn documents(&self) -> &Option<Box<Vec<Document>>> {
        &self.documents
    }

    /// Updates the page contents in memory.
    pub fn update_documents(&mut self, new: Vec<Document>) {
        self.documents = Some(new.into());
    }

    /// Updates the filesystem to match the in memory page.
    pub fn fwrite_all(&mut self) -> Result<(), WriteError> {
        self.fwrite_meta()?;
        self.fwrite_contents()?;

        Ok(())
    }

    /// Writes the memory contents onto the filesystem if loaded.
    pub fn fwrite_contents(&mut self) -> Result<(), WriteError> {
        if self.documents.is_none() {
            DatabasePath::Data.write(
                self.collection_name
                    .clone()
                    .into_file_name(self.metadata.pos as u32)
                    .as_str(),
                // TODO Marshall page docs into vec
                vec![],
            )?
        }

        Ok(())
    }

    /// Write the page metadata to the filesystem.
    pub fn fwrite_meta(&self) -> Result<(), WriteError> {
        let file_name = self
            .metadata
            .file_name(self.collection_name.clone().into_string());
        let bytes = self.metadata.as_bytes();

        let err = fs::write(DatabasePath::Data.file(&file_name), bytes).err();
        return match err {
            None => Ok(()),
            Some(e) => Err(e.into()),
        };
    }
}
