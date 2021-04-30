use std::collections::HashMap;

use serde_json::{json, Value};

use crate::graph::graph::SerializationError;
use crate::lib::bson::JsonObject;
use crate::server::http::context::HttpContext;

pub type DirectiveResult = Result<JsonObject, DirectiveError>;
pub type DirectiveList = HashMap<String, &'static dyn Directive>;

/// A prefixed JSON key that executes a database query.
pub trait Directive: Sync + Send {
  /// The key name. Not the actual formatted JSON key.
  fn key(&self) -> &str;
  /// Execute the directive's action.
  fn exec(&self, ctx: HttpContext) -> DirectiveResult;
}

pub struct DirectiveData<'a>(&'a JsonObject);

impl DirectiveData<'_> {
  pub fn new(data: &JsonObject) -> DirectiveData {
    DirectiveData { 0: data }
  }

  pub fn data(&self) -> &JsonObject {
    self.0
  }

  /// Attempts to retrieve an optional key from an object.
  pub fn get_optional(&self, key: &str) -> Option<&Value> {
    self.0.get(key)
  }

  pub fn get_required(&self, key: &str) -> Result<&Value, DirectiveError> {
    let data = self.0.get(key);
    return match data {
      Some(v) => Ok(v),
      None => Err(DirectiveError::MissingKey(key.to_string())),
    };
  }
}

/// Wrapper to execute an action on each directive object in the directive array.
pub struct DirectiveDataSet(Vec<JsonObject>);

impl DirectiveDataSet {
  pub fn new(data: Vec<JsonObject>) -> DirectiveDataSet {
    DirectiveDataSet { 0: data }
  }

  /// Dispatches the directive on each object in the directive array.
  pub fn dispatch<T>(
    &self,
    action: fn(&DirectiveData) -> Result<T, DirectiveError>,
  ) -> Result<Vec<T>, DirectiveError> {
    let mut acc: Vec<T> = Vec::new();

    for o in &self.0 {
      let data = &DirectiveData::new(o);
      acc.push(action(data)?);
    }

    Ok(acc)
  }
}

/// Type of directive error.
pub enum DirectiveError {
  /// A required JSON key is missing.
  MissingKey(String),
  /// Serialization error while executing the directive.
  Serialization(SerializationError),
  /// Expected an array for the directive value.
  ExpectedArray,
  /// Expected an object for the directive value.
  ExpectedObject,
}

impl DirectiveError {
  pub fn get_message(&self) -> String {
    return match self {
      DirectiveError::MissingKey(key) => format!("Missing required key: {}", key),
      DirectiveError::Serialization(_) => "Node serialization error".into(),
      DirectiveError::ExpectedArray => "Expected directive data to be an array".into(),
      DirectiveError::ExpectedObject => "Expected directive data be an object".into(),
    };
  }

  /// Returns a JSON object containing the error details.
  pub fn get_data(&self) -> Value {
    return match self {
      DirectiveError::MissingKey(key) => json!({ "key": key }),
      DirectiveError::Serialization(v) => Value::Object(v.into()),
      DirectiveError::ExpectedArray => json!({}),
      DirectiveError::ExpectedObject => json!({}),
    };
  }
}

/// Creates a new JSON object representing a directive error.
pub fn new_error_object(directive_key: &str, err: DirectiveError) -> JsonObject {
  let mut obj = JsonObject::new();

  obj.insert(
    "directive".to_string(),
    Value::String(directive_key.to_string()),
  );

  let mut data = JsonObject::new();

  match err.get_data() {
    Value::Object(mut v) => data.append(&mut v),
    _ => {}
  };

  obj.append(&mut data);

  obj
}
