use crate::lib::json::JsonObject;
use rocket::http::{ContentType, Status};
use rocket::Response;
use std::io::Cursor;

/// REST API response type.
pub enum ResponseFormat {
	/// A plaintext table for CLI usage.
	Table,
	/// JSON object.
	JSON,
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

/// Builds an API error response in JSON format.
pub fn new_json_error<'a>(
	format: ResponseFormat,
	status: Status,
	msg: &str,
	data: &JsonObject,
) -> Response<'a> {
	build_response(
		ContentType::from(format),
		serde_json::to_string(data).unwrap_or("{}".into()),
		status,
	)
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
