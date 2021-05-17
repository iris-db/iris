use crate::lib::bson::JsonObject;
A
use rocket::http::Status;
use serde_json::{json, Value};
use std::convert::TryFrom;

/// Utility struct for easily getting required or optional data.
pub struct SmartJsonObject<'a>(pub &'a JsonObject);

impl<'a, 'b: 'a> TryFrom<&'b Value> for SmartJsonObject<'b> {
	type Error = &'static str;

	fn try_from(value: &'b Value) -> Result<Self, Self::Error> {
		if value.is_object() {
			Ok(SmartJsonObject(value.as_object().unwrap()))
		} else {
			Err("JSON value is not an object")
		}
	}
}

/// Error when a JSON key is missing.
#[derive(Debug)]
pub struct KeyNotFoundError(pub String);

impl ToResponse for KeyNotFoundError {
	fn marshall(&self) -> String {
		json!({ "key": self.0 }).to_string()
	}

	fn status(&self) -> Status {
		Status::BadRequest
	}
}

impl SmartJsonObject<'_> {
	/// Attempts to get an owned value.
	pub fn get_owned(&self, key: &str) -> Result<Value, KeyNotFoundError> {
		return match self.0.get(key).map(|v| v.clone()) {
			Some(v) => Ok(v),
			None => Err(KeyNotFoundError(key.to_string())),
		};
	}

	/// Attempts to get a borrowed value.
	pub fn get_ref(&self, key: &str) -> Result<&Value, KeyNotFoundError> {
		return match self.0.get(key) {
			Some(v) => Ok(v),
			None => Err(KeyNotFoundError(key.to_string())),
		};
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[test]
	pub fn test_get_owned() {
		let json = json!({ "Graph": "default" });

		let obj = SmartJsonObject::try_from(&json).expect("Unable to convert json object");

		let v = obj.get_owned("Graph").expect("Could not get Graph key");

		assert_eq!(
			v.as_str().expect("Could not convert Graph value to string"),
			"default"
		);
	}
}
