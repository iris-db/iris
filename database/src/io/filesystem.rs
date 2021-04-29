use std::fs;

#[allow(dead_code)]
const ROOT_PATH: &str = "iris";

pub const DATA_PATH: &str = "iris/data";
pub const TEMP_PATH: &str = "iris/temp";

pub const PATHS: &[&str] = &[DATA_PATH, TEMP_PATH];

/// Initializes all of the directories for the database process.
pub fn prepare() {
  for path in PATHS {
    fs::create_dir_all(path).err().and_then(|e| -> Option<()> {
      panic!("{}", e.to_string());
    });
  }
}

pub fn open_at_data_path(file_name: &str) {}

#[cfg(test)]
/// Utility functions for clean up.
pub mod test_utils {
  use super::*;

  /// Destroys file system directories.
  pub fn destroy() {
    fs::remove_dir_all(ROOT_PATH)
      .err()
      .and_then(|e| -> Option<()> {
        panic!("{}", e.to_string());
      });
  }
}
