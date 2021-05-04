use chrono::{DateTime, Utc};
use std::time::SystemTime;

/// Prepends the current time to a string.
pub fn prepend_time(s: &str) -> String {
	let mut t = current_time();

	t.push_str(&*format!(" {}", s));

	t
}

/// Gets the current time as a string.
pub fn current_time() -> String {
	let st = SystemTime::now();
	let dt: DateTime<Utc> = st.clone().into();
	format!("{}", dt.format("%+"))
}
