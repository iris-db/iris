use std::cmp::Ordering;
use std::collections::HashMap;

use serde_json::{json, Value};

use crate::aql::context::HttpContext;
use crate::graph::graph::SerializationError;
use crate::lib::bson::JsonObject;

pub type DirectiveResult = Result<JsonObject, DirectiveErrorType>;

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

  pub fn get_required(&self, key: &str) -> Result<&Value, DirectiveErrorType> {
    let data = self.0.get(key);
    return match data {
      Some(v) => Ok(v),
      None => Err(DirectiveErrorType::MissingKey(key.to_string())),
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
    action: fn(&DirectiveData) -> Result<T, DirectiveErrorType>,
  ) -> Result<Vec<T>, DirectiveErrorType> {
    let mut acc: Vec<T> = Vec::new();

    for o in &self.0 {
      let data = &DirectiveData::new(o);
      acc.push(action(data)?);
    }

    Ok(acc)
  }
}

/// The result of data extraction from the POST body for the directive.
pub enum DirectiveDataExtraction<'a> {
  /// JSON object.
  Object(JsonObject),
  /// JSON array.
  Array(Vec<JsonObject>),
  /// Other JSON type, such as a number.
  Other(&'a Value),
}

/// Error while executing a directive.
pub struct DirectiveError<'a> {
  pub directive_key: &'a str,
  pub err_type: DirectiveErrorType,
}

// Type of directive error.
pub enum DirectiveErrorType {
  /// A required JSON key is missing.
  MissingKey(String),
  /// Serialization error while executing the directive.
  Serialization(SerializationError),
  /// Expected an array for the directive value.
  ExpectedArray,
  /// Expected an object for the directive value.
  ExpectedObject,
}

impl Into<JsonObject> for DirectiveError<'_> {
  fn into(self) -> JsonObject {
    let err_type = &self.err_type;

    let data: Value = match err_type {
      DirectiveErrorType::MissingKey(v) => json!({ "key": v }),
      DirectiveErrorType::ExpectedArray => json!({ "type": "array" }),
      DirectiveErrorType::ExpectedObject => json!({ "type": "object" }),
      DirectiveErrorType::Serialization(v) => Value::Object(v.into()),
    };

    let mut json = JsonObject::new();

    json.insert("directive".to_string(), json!(self.directive_key));
    json.insert("msg".to_string(), get_err_message(err_type));
    json.insert("data".to_string(), data);

    json
  }
}

fn get_err_message(t: &DirectiveErrorType) -> Value {
  return match t {
    DirectiveErrorType::MissingKey(v) => format!("Missing key: {}", v).into(),
    DirectiveErrorType::ExpectedArray => format!("Invalid type: array").into(),
    DirectiveErrorType::ExpectedObject => format!("Invalid type: object").into(),
    DirectiveErrorType::Serialization(_) => "Serialization error".to_string().into(),
  };
}

impl Ord for dyn Directive {
  fn cmp(&self, other: &Self) -> Ordering {
    self.key().cmp(other.key())
  }
}

impl PartialOrd for dyn Directive {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.key().partial_cmp(other.key())
  }
}

impl Eq for dyn Directive {}

impl PartialEq for dyn Directive {
  fn eq(&self, other: &Self) -> bool {
    self.key().eq(other.key())
  }
}
