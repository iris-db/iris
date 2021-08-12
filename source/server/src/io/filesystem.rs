use crate::io::file_descriptor::FileDescriptor;
use crate::io::path::DatabasePath;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;
use std::{fs, io};

/// An abstraction over the OS filesystem that allows for different implementations of common
/// operations.
pub trait Filesystem {
    /// Creates a directory if it does not exist.
    fn create_dir<T>(&self, name: T) -> io::Result<()>
    where
        T: AsRef<str>;

    /// Creates a new file if it does not exist.
    fn create_file(&self, f: &FileDescriptor) -> io::Result<()>;

    /// Check if a file exists.
    fn file_exists(&self, f: &FileDescriptor) -> bool;

    /// Overwrites a file with the new contents.
    fn overwrite(&self, f: &FileDescriptor, bytes: Vec<u8>) -> io::Result<()>;

    /// Appends bytes to the end of a file.
    fn append(&self, f: &FileDescriptor, bytes: Vec<u8>) -> io::Result<()>;
}

/// The native filesystem used in the production build.
pub struct StdFs;

impl Filesystem for StdFs {
    fn create_dir<T>(&self, name: T) -> Result<(), io::Error>
    where
        T: AsRef<str>,
    {
        fs::create_dir_all(name.as_ref())?;
        Ok(())
    }

    fn create_file(&self, f: &FileDescriptor) -> io::Result<()> {
        fs::write(f.relative_path(), Vec::new())?;
        Ok(())
    }

    fn file_exists(&self, f: &FileDescriptor) -> bool {
        Path::new(&f.relative_path()).exists()
    }

    fn overwrite(&self, f: &FileDescriptor, bytes: Vec<u8>) -> io::Result<()> {
        fs::write(&f.relative_path(), bytes)?;
        Ok(())
    }

    fn append(&self, f: &FileDescriptor, bytes: Vec<u8>) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .create(true)
            .open(&f.relative_path())?;

        file.write(&*bytes)?;

        Ok(())
    }
}

/// <strong>SHOULD ALWAYS BE USED WHEN TESTING</strong>
///
/// A filesystem that does not modify the OS filesystem in any way.
pub struct InactiveFs;

impl Filesystem for InactiveFs {
    fn create_dir<T>(&self, _name: T) -> io::Result<()>
    where
        T: AsRef<str>,
    {
        Ok(())
    }

    fn create_file(&self, _f: &FileDescriptor) -> io::Result<()> {
        Ok(())
    }

    fn file_exists(&self, _f: &FileDescriptor) -> bool {
        true
    }

    fn overwrite(&self, _f: &FileDescriptor, _bytes: Vec<u8>) -> io::Result<()> {
        Ok(())
    }

    fn append(&self, _f: &FileDescriptor, _bytes: Vec<u8>) -> io::Result<()> {
        Ok(())
    }
}
