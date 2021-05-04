use std::fs::OpenOptions;
use std::io::Write;
use std::slice::Iter;
use std::{fs, io};

pub const ROOT_PATH: &str = "iris";

/// Initializes all of the directories for the iris process.
pub fn prepare() {
	for path in DatabasePath::iter() {
		match fs::create_dir_all(path.path()) {
			Err(e) => panic!("{}", e),
			_ => {}
		}
	}
}

pub fn destroy() {}

/// Database storage paths.
pub enum DatabasePath {
	/// Graph data.
	Data,
	/// Temporary files that will be eventually removed from the disk.
	Temp,
	/// Log files.
	Logs,
}

impl DatabasePath {
	/// Returns an iterator of each path.
	pub fn iter() -> Iter<'static, DatabasePath> {
		use DatabasePath::*;

		static PATHS: [DatabasePath; 3] = [Data, Temp, Logs];

		PATHS.iter()
	}

	/// Writes a set of bytes to a file and overwrites its previous contents.
	pub fn write(&self, file: &str, contents: Vec<u8>) -> Result<(), io::Error> {
		fs::write(self.file(file), contents)
	}

	/// Writes a set of bytes to a file and appends it to the file.
	pub fn append(&self, file: &str, contents: Vec<u8>) -> Result<(), io::Error> {
		let mut f = OpenOptions::new()
			.append(true)
			.write(true)
			.create(true)
			.open(self.file(file))?;

		f.write(&*contents)?;

		Ok(())
	}

	/// Returns the relative path of a file in the directory.
	///
	/// For example (Log path):
	///
	/// * `Input` - object.meta
	/// * `Output` - iris/logs/object.meta
	pub fn file(&self, file: &str) -> String {
		format!("{}/{}", self.path(), file)
	}

	/// Gets the path.
	pub fn path(&self) -> String {
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

#[cfg(tests)]
mod tests {
	#[test]
	fn test_write() {}
}
