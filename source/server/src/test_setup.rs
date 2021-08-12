///! Deletes database runtime directory after executing tests.
use std::fs;

use crate::io::path;
use crate::io::path::ROOT_PATH;

#[cfg(test)]
#[ctor::ctor]
fn prepare_filesystem() {
    path::prepare();
}

#[cfg(test)]
#[ctor::dtor]
fn cleanup_filesystem() {
    let _ = fs::remove_dir_all(ROOT_PATH);
}
