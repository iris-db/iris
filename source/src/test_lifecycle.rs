use std::fs;

use crate::io::filesystem;
use crate::io::filesystem::ROOT_PATH;

#[cfg(test)]
#[ctor::ctor]
fn prepare_filesystem() {
	filesystem::prepare();
}

#[cfg(test)]
#[ctor::dtor]
fn cleanup_filesystem() {
	let _ = fs::remove_dir_all(ROOT_PATH);
}
