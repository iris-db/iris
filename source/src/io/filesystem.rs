use std::fs;

#[allow(dead_code)]
pub const ROOT_PATH: &str = "iris";

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
