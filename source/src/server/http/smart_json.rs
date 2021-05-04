use crate::lib::bson::JsonObject;
use serde_json::Value;
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

impl SmartJsonObject<'_> {
	/// Attempts to get an owned value.
	pub fn get_owned<T>(&self, key: &str) -> Option<Value> {
		return match self.0.get(key) {
			Some(v) => Some(v.clone()),
			None => None,
		};
	}

	/// Attempts to get a borrowed value.
	pub fn get_ref(&self, key: &str) -> Option<&Value> {
		self.0.get(key)
	}
}
