#[derive(Debug, Clone)]
/// Utility for formatting a collection name into several useful filename formats.
pub struct CollectionNameFormatter(String);

impl CollectionNameFormatter {
    pub fn new<T>(s: T) -> Self
    where
        T: AsRef<str>,
    {
        CollectionNameFormatter(s.as_ref().into())
    }

    /// Get the raw collection name.
    pub fn original(&self) -> &String {
        &self.0
    }

    /// Convert to the raw collection name.
    pub fn into_original(self) -> String {
        self.0
    }

    /// Get the page file name based on the page id.
    pub fn as_page_file_name(&self, page_id: u32) -> String {
        format!("{}.{}", self.0, page_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collection_name_as_page_file_name() {
        let col_name = CollectionNameFormatter::new("users");

        let page_file_name = col_name.as_page_file_name(16);

        assert_eq!(page_file_name, "users.16");
    }
}
