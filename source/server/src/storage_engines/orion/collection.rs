use crate::page::page::Page;

#[derive(Debug, Clone)]
pub struct CollectionName(String);

impl From<String> for CollectionName {
    fn from(s: String) -> Self {
        CollectionName(s)
    }
}

impl CollectionName {
    /// Get the raw collection name.
    pub fn as_string(&self) -> &String {
        &self.0
    }

    /// Convert to the raw collection name.
    pub fn into_string(self) -> String {
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
    pages: Option<Vec<&'a Page>>,
}

impl Collection<'_> {
    pub fn new(name: CollectionName) -> Self {
        todo!()
    }

    pub fn name(&self) -> &CollectionName {
        &self.name
    }

    pub fn dispatch_query(&self) {}
}
