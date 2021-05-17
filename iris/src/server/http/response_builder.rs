use crate::server::http::errors::RequestError;
use rocket::http::{ContentType, Status};
use rocket::Response;
use std::io::Cursor;

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
pub fn new_response<'a>(content_type: ContentType, data: String, status: Status) -> Response<'a> {
	Response::build()
		.header(content_type)
		.sized_body(Cursor::new(data))
		.status(status)
		.ok::<&str>()
		.unwrap()
}
