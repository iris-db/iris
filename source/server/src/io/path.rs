use crate::io::filesystem::Filesystem;
use std::fs::OpenOptions;
use std::io::Write;
use std::slice::Iter;
use std::{fs, io};

pub const ROOT_PATH: &str = "iris";

/// Initializes all of the directories for the src process.
pub fn prepare() {
    for path in DatabasePath::paths() {
        match fs::create_dir_all(path.path_name()) {
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}

/// src storage paths.
pub enum DatabasePath {
    /// Graph data.
    Data,
    /// Temporary files that will be eventually removed from the disk.
    Temp,
    /// Log files.
    Logs,
}

impl DatabasePath {
    /// Creates the directory on the specified filesystem.
    pub fn init<F>(&self, fs: &F)
    where
        F: Filesystem,
    {
        match fs::create_dir_all(self.path_name()) {
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    /// Returns an iterator of each path.
    pub fn paths() -> Iter<'static, DatabasePath> {
        use DatabasePath::*;

        static PATHS: [DatabasePath; 3] = [Data, Temp, Logs];

        PATHS.iter()
    }

    /// Returns the relative path of a file in the directory.
    ///
    /// For example (Log path):
    ///
    /// * `Input` - object.meta
    /// * `Output` - src/logs/object.meta
    pub fn file<T>(&self, file: T) -> String
    where
        T: AsRef<str>,
    {
        format!("{}/{}", self.path_name(), file.as_ref())
    }

    /// The name of the directory relative to the database root.
    pub fn path_name(&self) -> String {
        fn cat_root(path: &str) -> String {
            format!("{}/{}", ROOT_PATH, path)
        }

        return match self {
            DatabasePath::Data => cat_root("data"),
            DatabasePath::Temp => cat_root("temp"),
            DatabasePath::Logs => cat_root("logs"),
        };
    }
}
