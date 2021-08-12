use crate::io::path::DatabasePath;

/// Describes where a file lives on the filesystem.
pub struct FileDescriptor {
    pub path: DatabasePath,
    pub name: String,
}

impl FileDescriptor {
    /// The file path relative to the server binary.
    pub fn relative_path(&self) -> String {
        self.path.file(&self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_relative_path() {
        let fd = FileDescriptor {
            path: DatabasePath::Data,
            name: "pageIndex.0".to_string(),
        };

        let relative_path = fd.relative_path();

        assert_eq!(relative_path, "data/pageIndex.0");
    }
}
