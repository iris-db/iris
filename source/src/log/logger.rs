use crate::io::filesystem::DatabasePath;
use crate::lib::bson::JsonObject;
use crate::log::util::current_time;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, io};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// A log of a database event.
struct Log {
	/// The time the event happened.
	time: String,
	/// A brief message explaining what happened.
	msg: String,
	/// The session id that the event occurred from.
	session_id: Option<String>,
	/// Data for the logged event.
	data: JsonObject,
}

/// The severity of an event.
enum LogSeverity {
	/// Successful operation.
	Ok,
	/// Warning.
	Warn,
	/// A recoverable error occurred.
	Error,
	/// A system critical error that can possibly be recovered from.
	Critical,
	/// An error that crashes the database.
	Fatal,
}

impl Log {
	fn new<D>(time: String, msg: &str, session_id: Option<Uuid>, data: Option<D>) -> Log
	where
		D: Serialize + Deserialize<'static>,
	{
		Log {
			msg: msg.to_string(),
			time,
			session_id: session_id.map(|u| u.to_string()),
			data: serde_json::to_value(data)
				.unwrap_or(Value::from(JsonObject::new()))
				.as_object()
				.unwrap()
				.clone(),
		}
	}
}

impl ToString for Log {
	fn to_string(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}

/// Standard log result.
pub struct SLogResult {
	/// The formatted message.
	pub msg: String,
	/// The time (ISO-8106)
	pub time: String,
}

/// Prints a log message to STDOUT.
///
/// Returns current time (ISO-8106) as string
pub fn s_log(msg: &str) -> SLogResult {
	let time = current_time();

	let msg = format!("{} {}", time, msg);

	println!("{}", msg);

	SLogResult { msg, time }
}

/// A standard log message.
///
/// Each log function will print its contents to STDOUT and save it to the current log file.
pub fn log(msg: &str, session_id: Option<Uuid>, data: Option<JsonObject>) {
	let SLogResult { time, .. } = s_log(msg);

	let log_file = match current_log_file() {
		Ok(file) => file,
		Err(_) => return,
	};

	let log = Log::new(time, msg, session_id, data);

	// Attempt to write to the filesystem.
	let _ = DatabasePath::Logs.write(&*log_file, log.to_string().into_bytes());
}

/// Logs a warning message.
pub fn warn() {}

/// Logs an error message.
pub fn err() {}

/// Logs a fatal error.
pub fn fatal_err() {}

/// Gets the current log file to write to.
///
/// A new log file is generated every 24h.
fn current_log_file() -> Result<String, io::Error> {
	let res = fs::read_dir(DatabasePath::Logs.path())?;

	let mut file_names: Vec<String> = Vec::new();

	for entry in res {
		let entry = entry?;

		let path = entry.path();

		if path.is_file() {
			let file_name = path.file_name();
			let file_name = match file_name {
				Some(name) => name,
				None => continue,
			};

			let file_name = file_name.to_str();
			match file_name {
				Some(name) => file_names.push(name.to_string()),
				None => continue,
			};
		}
	}

	Ok("LOGF".to_string())
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::Path;

	#[test]
	fn test_s_log() {
		let msg = "CONNPOOL Connecting to shard C-00-00";

		assert!(Path::new(&*DatabasePath::Logs.file("LOGF")).exists())
	}
}
