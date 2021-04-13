use serde_json::{Map, Value};

/// Alias for a serde map `Map<String, Value>`.
pub type JsonObject = Map<String, Value>;
