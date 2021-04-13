/// Result of a disk serialization operation.
pub struct SerializationResult {
    /// Serialization time in milliseconds.
    pub time: u128,
}

/// Result of a disk read operation.
pub struct DeserializationResult<T> {
    /// Serialization time in milliseconds.
    pub time: u128,
    /// Loaded data.
    pub data: T,
}

/// Can be saved to disk.
pub trait Serializable<T> {
    /// Persist data to the disk.
    fn write(&self) -> SerializationResult;
    /// Load data from the disk.
    fn read(&self) -> DeserializationResult<T>;
    /// Data path relative to the root database directory.
    fn get_data_path(&self) -> String;
}
