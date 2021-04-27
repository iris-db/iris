use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

use serde_json::{json, Value};

use crate::aql::context::AqlContext;
use crate::graph::graph::SerializationError;
use crate::lib::bson::{values_to_objects, JsonObject};

pub type DirectiveResult<'a> = Result<JsonObject, DirectiveErrorType<'a>>;

pub type DirectiveList = HashMap<String, &'static dyn Directive>;

/// A prefixed JSON key that executes a database query.
pub trait Directive: Sync + Send {
  /// The key name. Not the actual formatted JSON key.
  fn key(&self) -> &str;
  /// Execute the directive's action.
  fn exec(&self, ctx: &mut AqlContext) -> DirectiveResult;
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
  pub err_type: DirectiveErrorType<'a>,
}

// Type of directive error.
pub enum DirectiveErrorType<'a> {
  /// A required JSON key is missing.
  MissingKey(&'a str),
  /// Wrong JSON type for the directive value.
  InvalidType(&'a str),
  /// Serialization error while executing the directive.
  Serialization(SerializationError),
}

impl Into<JsonObject> for DirectiveError<'_> {
  fn into(self) -> JsonObject {
    let err_type = &self.err_type;

    let data: Value = match err_type {
      DirectiveErrorType::MissingKey(v) => json!({ "key": v }),
      DirectiveErrorType::InvalidType(v) => json!({ "type": v }),
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
    DirectiveErrorType::InvalidType(v) => format!("Invalid type: {}", v).into(),
    DirectiveErrorType::Serialization(_) => "Serialization error".to_string().into(),
  };
}

/// Extracts directive data by looking up the directive JSON key's value on the request body.
pub fn extract_directive_data<'a>(
  directive: &dyn Directive,
  data: &'a JsonObject,
) -> DirectiveDataExtraction<'a> {
  let key = directive.key();

  let data = data.get(key).unwrap();

  return match data {
    Value::Array(v) => DirectiveDataExtraction::Array(values_to_objects(v)),
    Value::Object(v) => DirectiveDataExtraction::Object(v.clone()),
    v => DirectiveDataExtraction::Other(v),
  };
}

/// Attempts to retrieve the wanted JSON data type returning an error if its not the expected type.
pub fn match_directive_data() {}

// impl dyn Directive {
//   pub fn error(&self, err_type: DirectiveErrorType) -> DirectiveResult {
//     Err(DirectiveError {
//       directive_key: self.key(),
//       err_type,
//     })
//   }
// }

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
