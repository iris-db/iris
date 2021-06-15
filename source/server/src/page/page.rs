use crate::page::decode::Decoder;

/// The maximum amount of data that is able to fit on a single page.
///
/// The standard maximum is 2MB.
pub const MAX_PAGE_SIZE: usize = 2E6 as usize;
/// File extension of the page metadata.
pub const META_PAGE_EXT: &str = "meta";

/// A set of data limited to the max page size.
pub struct Page<'a, T> {
    /// Filesystem path relative to the database data path.
    path: String,
    /// Decoding strategy for the page.
    decoder: &'a dyn Decoder<T>,
    /// Page data loaded in memory.
    contents: Option<Vec<T>>,
}

impl<'a, T> Page<'a, T> {
    /// Creates a new page from a path relative to the database data path.
    pub fn new(path: String, decoder: &'a dyn Decoder<T>) -> Self {
        Page {
            path,
            decoder,
            contents: Some(Vec::new()),
        }
    }

    /// Loads the page contents into memory.
    pub fn load(&self) {}

    /// Frees the page contents from memory.
    pub fn release(&self) {}

    /// Get the page contents if loaded in memory.
    pub fn contents(&self) -> &Option<Vec<T>> {
        &self.contents
    }

    /// Update the page contents in memory and on the disk.
    pub fn rewrite_contents(&mut self, new: Vec<u8>) {
        self.contents = new.into();
        // Rewrite to disk
    }
}
