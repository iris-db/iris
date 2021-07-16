use crate::page::page::Page;
use crate::storage_engines::orion::document::Document;
use std::borrow::Borrow;

pub struct CollectionName(String);

impl From<String> for CollectionName {
    fn from(s: String) -> Self {
        CollectionName(s)
    }
}

impl CollectionName {
    /// Get the raw collection name.
    pub fn name(&self) -> &String {
        &self.0
    }

    /// Convert to the raw collection name.
    pub fn into_name(self) -> String {
        self.0
    }

    /// Get the page file name based on the page id.
    pub fn into_file_name(self, page_id: u32) -> String {
        format!("{}.{}", self.0, page_id)
    }
}

/// An abstraction over data pages.
pub struct Collection<'a> {
    name: CollectionName,
    page: Option<&'a Page<'a>>,
}

impl Collection<'_> {
    pub fn new(name: CollectionName) -> Self {
        let mut col = Collection { name, page: None };
        col.page = Some(&Page::new(&col).into_ok());

        col
    }

    pub fn name(&self) -> &CollectionName {
        &self.name
    }

    pub fn dispatch_query(&self) {}
}
