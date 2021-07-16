use crate::io::filesystem::DatabasePath;
use crate::lib::json::bsonio;
use crate::lib::json::types::JsonObject;
use crate::page::error::{ReadError, WriteError};
use crate::page::io::get_meta_path;
use crate::page::metadata::PageMetadata;
use crate::storage_engines::orion::collection::Collection;
use crate::storage_engines::orion::document::Document;
use std::fs;

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
pub struct Page<'a> {
    /// Filesystem path relative to the database data path.
    collection_name: String,
    /// The page id for the collection.
    metadata: PageMetadata,
    /// Page data loaded in memory.
    documents: Option<Box<Vec<Document>>>,
}

impl Page<'_> {
    /// Creates a new page on the filesystem if it does not exist.
    pub fn create(collection_name: String) -> Self {
        let mut page = Page {
            collection_name,
            metadata: PageMetadata::new(),
            documents: None,
        };

        page.fwrite_all();

        page
    }

    /// Loads the page contents into memory.
    pub fn load_contents_into_memory(&mut self) -> Result<(), ReadError> {
        let mut file = &self.file;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let documents = bsonio::decoder::decode_json_objects(buf)?;
        self.documents = Some(documents.into_iter().map(Into::into).collect());

        Ok(())
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
    pub fn fwrite_all(&mut self) {
        self.fwrite_contents();
        self.fwrite_meta();
    }

    /// Writes the memory contents onto the filesystem if loaded.
    pub fn fwrite_contents(&mut self) {
        if self.documents.is_none() {
            DatabasePath::Data.write(
                self.collection.name().into_file_name(self.id).into(),
                // TODO Marshall page docs into vec
                vec![],
            );
        }
    }

    /// Write the page metadata to the filesystem.
    pub fn fwrite_meta(&self) {
        let file_name = self.metadata.file_name(self.collection_name.clone());
        let bytes = self.metadata.as_bytes();

        fs::write(DatabasePath::Data.file(&file_name), bytes);
    }
}
