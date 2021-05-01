use std::fs;

#[allow(dead_code)]
const ROOT_PATH: &str = "iris";

pub const DATA_PATH: &str = "iris/data";
pub const TEMP_PATH: &str = "iris/temp";

pub const PATHS: &[&str] = &[DATA_PATH, TEMP_PATH];

/// Initializes all of the directories for the source process.
pub fn prepare() {
  for path in PATHS {
    match fs::create_dir_all(path) {
      Err(e) => panic!("{}", e),
      _ => {}
    }
  }
}

#[cfg(test)]
/// Utility functions for clean up.
pub mod test_utils {
  use super::*;
  use std::io::Error;
  use std::path::Path;

  #[macro_export]
  macro_rules! use_test_filesystem {
    () => {
      let _fs = crate::io::filesystem::test_utils::TestFilesystem::new();
    };
  }

  /// Destroys file system completing a test.
  pub struct TestFilesystem;

  impl TestFilesystem {
    /// Initializes the test filesystem.
    pub fn new() -> Box<TestFilesystem> {
      for path in PATHS {
        let _ = fs::create_dir_all(path);
      }

      Box::from(TestFilesystem)
    }
  }

  impl Drop for TestFilesystem {
    fn drop(&mut self) {
      destroy()
    }
  }

  /// Destroys file system directories.
  pub fn destroy() {
    let _ = fs::remove_dir_all(ROOT_PATH);
  }
}
