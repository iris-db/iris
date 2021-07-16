use bson::de::Error;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
/// Error that occurs when attempting to read BSON documents from a page.
pub enum ReadError {
    /// Io error.
    Io(io::Error),
    /// Error while trying to deserialize a document.
    CorruptedBsonDocument(bson::de::Error),
    /// Not a UTF-8 header.
    CorruptedHeader(FromUtf8Error),
    /// Improper key value formatting.
    MalformedHeader,
}

impl From<bson::de::Error> for ReadError {
    fn from(e: Error) -> Self {
        ReadError::CorruptedBsonDocument(e)
    }
}

#[derive(Debug)]
/// Error that occurs when attempting to write data to a page.
pub enum WriteError {
    /// Could not create a new page.
    CouldNotCreatePage(io::Error),
    /// Io error.
    Io(io::Error),
    /// The data attempting to be written will overflow the page size.
    PageSizeExceeded(usize),
}

impl From<io::Error> for WriteError {
    fn from(e: io::Error) -> Self {
        WriteError::Io(e)
    }
}
