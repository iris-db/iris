use crate::io::logger::util::fmt_log_msg;
use serde::{Deserialize, Serialize};

/// Representation of a loggable database event.
#[derive(Serialize, Deserialize)]
pub struct Event {
	/// Severity of the event.
	severity: String,
	/// When the event occurred.
	time: String,
	/// Data representing the event.
	data: String,
	/// A UUIDv4 representing which server in a cluster the event occurred.
	server_id: String,
}

/// The severity of an event.
pub enum EventSeverity {
	/// Information message.
	Info,
	/// Warning.
	Warn,
	/// A recoverable error occurred.
	Error,
	/// A system critical error that can possibly be recovered from.
	Critical,
	/// An error that crashes the database.
	Fatal,
}

impl ToString for EventSeverity {
	fn to_string(&self) -> String {
		return match self {
			EventSeverity::Info => "INFO",
			EventSeverity::Warn => "WARN",
			EventSeverity::Error => "ERROR",
			EventSeverity::Critical => "CRITICAL",
			EventSeverity::Fatal => "FATAL",
		}
		.to_string();
	}
}

impl Event {
	/// Creates a new event with the current time.
	fn now(severity: EventSeverity, data: String, server_id: String) -> Event {
		Event {
			severity: severity.to_string(),
			time: util::current_time(),
			data,
			server_id,
		}
	}

	/// Creates a new Event.
	fn new(severity: EventSeverity, time: String, data: String, server_id: String) -> Event {
		Event {
			severity: severity.to_string(),
			time,
			data,
			server_id,
		}
	}
}

/// Logs an event, writing it to the filesystem.
pub fn f_log_e(e: Event) {}

/// Writes a message to STDOUT in log message format.
pub fn s_log(severity: EventSeverity, msg: &str) {
	println!("{}", fmt_log_msg(util::current_time(), severity, msg));
}

mod util {
	use crate::io::logger::EventSeverity;
	use chrono::{DateTime, Utc};
	use std::time::SystemTime;

	/// Prepends the current time to a string.
	pub fn fmt_log_msg(time: String, severity: EventSeverity, msg: &str) -> String {
		format!(
			"{} {} {}",
			time,
			// First character of the severity level.
			severity.to_string().chars().next().unwrap(),
			msg
		)
	}

	/// Gets the current time as a string (ISO-8106).
	pub fn current_time() -> String {
		let st = SystemTime::now();
		let dt: DateTime<Utc> = st.clone().into();
		format!("{}", dt.format("%+"))
	}
}
