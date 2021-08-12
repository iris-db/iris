use crate::page::page::Page;

/// A set of pages that represents a full or partial database collection.
pub struct PageSet {
    pages: Vec<Page>,
}

impl PageSet {
    /// Releases all pages from memory.
    pub fn release_all(&self) {
        todo!()
    }

    /// Releases the last recently used page from memory.
    pub fn release_lru() {
        todo!()
    }
}
