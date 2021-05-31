use crate::lib::json::JsonObject;
use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, Response};
use serde_json::json;
use std::io::Cursor;

pub const DEFAULT_RESPONSE_FORMAT: ResponseFormat = ResponseFormat::Table;

/// A REST api response.
///
/// * `0` - The response status
/// * `1` - The response body
pub struct ApiResponse<'a>(Status, &'a Vec<u8>);

impl<'a> ApiResponse<'a> {
	/// Creates a raw ApiResponse.
	pub fn new(status: Status, body: &'a Vec<u8>) -> Self {
		Self(status, body)
	}

	/// Marshals an API response into a Rocket response.
	pub fn marshal(&self, fmt: ResponseFormat) -> Response<'a> {
		Response::build()
			.header(ContentType::from(fmt))
			.sized_body(Cursor::new(self.1))
			.status(self.0)
			.ok::<()>()
			.unwrap()
	}
}

/// REST API response type.
pub enum ResponseFormat {
	/// A plaintext table for CLI usage.
	Table,
	/// JSON object.
	JSON,
}

impl From<&str> for ResponseFormat {
	fn from(s: &str) -> Self {
		return match s {
			"table" => Self::Table,
			"json" => Self::JSON,
			_ => DEFAULT_RESPONSE_FORMAT,
		};
	}
}

impl FromRequest<'_, '_> for ResponseFormat {
	type Error = ();

	fn from_request(request: &Request<'_>) -> Outcome<Self, Self::Error> {
		let keys: Vec<_> = request.headers().get("x-response-format").collect();
		if keys.len() != 1 {
			return Outcome::Success(DEFAULT_RESPONSE_FORMAT);
		}

		Outcome::Success(ResponseFormat::from(&*keys[0].to_lowercase()))
	}
}

impl From<ResponseFormat> for ContentType {
	fn from(f: ResponseFormat) -> Self {
		return match f {
			ResponseFormat::Table => ContentType::Plain,
			ResponseFormat::JSON => ContentType::JSON,
		};
	}
}

/// Converts an object into an api response.
pub trait ToResponse {
	/// Marshals the object into a string.
	fn marshall(&self) -> String;
	/// The status of the response if this object is converted.
	fn status(&self) -> Status;
}

/// Builds an API response from a convertible object.
pub fn from<'a, T>(content_type: ContentType, t: T) -> Response<'a>
where
	T: ToResponse,
{
	Response::build()
		.header(content_type)
		.sized_body(Cursor::new(t.marshall()))
		.status(t.status())
		.ok::<&str>()
		.unwrap()
}

/// Builds an API response as either plain text or JSON.
pub fn new_response<'a>(format: ResponseFormat, status: Status, data: String) -> Response<'a> {
	build_response(ContentType::from(format), data, status)
}

/// Builds an API error object.
pub fn json_error_object(msg: &str, data: &JsonObject) -> JsonObject {
	json!({
		"Message": msg,
		"Data": data
	})
	.as_object()
	.unwrap()
	.clone()
}

/// Builds a response from a content type header, a sized body, and a status code.
fn build_response<'a, T: 'a>(ct: ContentType, body: T, status: Status) -> Response<'a>
where
	T: AsRef<[u8]>,
{
	Response::build()
		.header(ct)
		.sized_body(Cursor::new(body))
		.status(status)
		.ok::<()>()
		.unwrap()
}
