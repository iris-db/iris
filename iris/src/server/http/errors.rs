use crate::server::http::response_builder::ToResponse;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::{json, Value};

pub enum RequestError {
	ImproperType(String),
}

impl ToResponse for RequestError {
	fn marshall(&self) -> String {
		return match self {
			RequestError::ImproperType(expected) => "",
		}
		.to_string();
	}

	fn status(&self) -> Status {
		todo!()
	}
}
