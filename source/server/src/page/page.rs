use crate::lib::json::types::JsonObject;

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
    path: String,
    /// Page data loaded in memory.
    contents: Option<Vec<JsonObject>>,
}

impl Page {
    /// Creates a new page from a path relative to the database data path.
    pub fn link(path: String) -> Self {
        Page {
            path,
            contents: None,
        }
    }

    /// Loads the page contents into memory.
    pub fn load(&self) {}

    /// Frees the page contents from memory.
    pub fn release(&self) {}

    /// Get the page contents if loaded in memory.
    pub fn contents(&self) -> &Option<Vec<JsonObject>> {
        &self.contents
    }

    /// Updates the page contents in memory.
    pub fn update_contents(new: Vec<JsonObject>) {}

    pub fn sync() {}

    /// Writes the memory contents onto the filesystem if loaded.
    pub fn fwrite(&mut self) {
        if self.contents.is_none() {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_memory_loader() {
        let page = Page::link("/".into());
        page.load();
    }

    fn view_state_controller() {}
}
