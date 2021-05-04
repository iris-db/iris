use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, io};

#[allow(dead_code)]
pub const ROOT_PATH: &str = "iris";

pub const DATA_PATH: &str = "iris/data";
pub const TEMP_PATH: &str = "iris/temp";
pub const LOGS_PATH: &str = "iris/logs";

pub const PATHS: &[&str] = &[DATA_PATH, TEMP_PATH, LOGS_PATH];

/// Initializes all of the directories for the source process.
pub fn prepare() {
	for path in PATHS {
		match fs::create_dir_all(path) {
			Err(e) => panic!("{}", e),
			_ => {}
		}
	}
}

/// Database storage paths.
pub enum Paths {
	/// Graph data.
	Data,
	/// Temporary files that will be eventually removed from the disk.
	Temp,
	/// Log files.
	Logs,
}

impl Paths {
	/// Writes a set of bytes to a file and overwrites its previous contents.
	pub fn write(&self, contents: Vec<u8>) -> Result<(), io::Error> {
		fs::write(self.path(), contents)
	}

	/// Writes a set of bytes to a file and appends it to the file.
	pub fn append(&self, contents: Vec<u8>) -> Result<(), io::Error> {
		let mut f = OpenOptions::new()
			.append(true)
			.write(true)
			.open(self.path())?;

		f.write(&*contents)?;

		Ok(())
	}

	/// Gets the relative path.
	fn path(&self) -> String {
		fn cat_root(path: &str) -> String {
			format!("{}/{}", ROOT_PATH, path)
		}

		return match self {
			Paths::Data => cat_root("data"),
			Paths::Temp => cat_root("temp"),
			Paths::Logs => cat_root("logs"),
		};
	}
}
