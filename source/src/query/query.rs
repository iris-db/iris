use std::collections::HashMap;

use serde_json::{json, Value};

use crate::graph::graph::Graph;
use crate::io::page;
use crate::io::page::WriteError;
use crate::lib::bson::JsonObject;

pub type QueryResult<R> = Result<R, Error>;

/// A query that can be executed on the database.
pub trait Query<R>: Sync + Send {
  /// Execute the query on a graph.
  fn exec(graph: &mut Graph, data: &JsonObject) -> QueryResult<R>;
}

/// Error that occurs while executing a query.
pub enum Error {
  /// A required JSON key is missing.
  MissingKey(String),
  /// Error while writing to a page.
  PageWrite(page::WriteError),
  /// Expected an array for the directive value.
  ExpectedArray,
  /// Expected an object for the directive value.
  ExpectedObject,
}

impl From<page::WriteError> for Error {
  fn from(e: WriteError) -> Self {
    Error::PageWrite(e)
  }
}

impl Error {
  pub fn get_message(&self) -> String {
    return match self {
      Error::MissingKey(key) => format!("Missing required key: {}", key),
      Error::ExpectedArray => "Expected directive data to be an array".into(),
      Error::ExpectedObject => "Expected directive data be an object".into(),
      Error::PageWrite(_) => "Could not save data to filesystem".into(),
    };
  }

  /// Returns a JSON object containing the error details.
  pub fn get_data(&self) -> Value {
    return match self {
      Error::MissingKey(key) => json!({ "key": key }),
      Error::ExpectedArray => json!({}),
      Error::ExpectedObject => json!({}),
      Error::PageWrite(_) => json!({}),
    };
  }
}

/// Creates a new JSON object representing a directive error.
pub fn new_error_object(directive_key: &str, err: Error) -> JsonObject {
  let mut obj = JsonObject::new();

  obj.insert(
    "directive".to_string(),
    Value::String(directive_key.to_string()),
  );

  obj.insert("msg".to_string(), Value::from(err.get_message()));

  let mut data = JsonObject::new();

  match err.get_data() {
    Value::Object(mut v) => data.append(&mut v),
    _ => {}
  };

  obj.insert("data".to_string(), data.into());

  obj
}
