use std::fs;

pub const DATA_PATH: &str = "iris/data";
pub const TEMP_PATH: &str = "iris/temp";

pub const PATHS: &[&str] = &[DATA_PATH, TEMP_PATH];

// Initializes all of the directories for the database process.
pub fn prepare() {
  for path in PATHS {
    fs::create_dir_all(path).err().and_then(|e| -> Option<()> {
      panic!("{}", e.to_string());
    });
  }
}

// Destroys file system directories.
pub fn destroy() {
  for path in PATHS {
    fs::remove_dir_all(path).err().and_then(|e| -> Option<()> {
      panic!("{}", e.to_string());
    });
  }
}
