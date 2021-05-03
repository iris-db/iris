use crate::lib::bson::JsonObject;
use std::cmp;

/// An executable action that performs an operation on a graph.
pub trait Command {
	fn exec() -> String;
}

/// Command configuration.
///
/// Example:
/// ```
/// $A = Insert -Group "User"
/// $B = Insert -Group "User"
/// $R = Relate -Name "IS_FRIENDS_WITH" -From $A -To $B
/// $R | Format-Json
/// Flush
/// ```
pub struct CommandConfig {
	/// Name of the command.
	name: String,
	/// Command flags.
	flags: Vec<Flag>,
	/// Expected fields to be returned by the operation. If the JSON array holding the results does
	/// not match the expected fields, an error is thrown.
	result_fields: Vec<JsonField>,
}

/// A JSON key that is expected on a JSON object.
/// `0` - The field name
/// `1` - Is required field
pub type JsonField = (String, bool);

/// A command argument.
pub struct Flag {
	name: String,
	flag_type: FlagType,
	required: bool,
}

/// A type of command flag.
pub enum FlagType {
	/// A string value.
	///
	/// `Command -Flag "String"`
	String(String),
	/// An array of strings. Strings are separated by a comma and the array ends when there is a
	/// whitespace.
	///
	/// `Command -Flag "A","B","C"`
	Array(Vec<String>),
	/// A valid JSON string. Can have quotes or be quote-less. The JSON string must be surrounded by
	/// single quotes.
	///
	/// `Command -Flag '{ key: "value" }'`
	/// `Command -Flag '{ "key": "value" }'`
	Json(String),
	/// A numerical value.
	///
	/// `Command -Flag 12 -Flag2 13.52`
	Number(f64),
	/// Boolean value.
	Boolean,
}

/// The result of completing a command.
pub struct CommandResult<'a> {
	pub fields: &'a Vec<&'a str>,
	pub data: &'a Vec<JsonObject>,
}

pub struct Pipe<'a> {
	left: CommandResult<'a>,
	right: CommandResult<'a>,
}

/// The amount of spaces used to separate a table column.
const TABLE_COL_SPACING: u32 = 8;

/// Formats a JSON object as a table.
fn fmt_table(result: &CommandResult) -> String {
	// Util functions.

	/// Calculates the spacing between a column.
	///
	/// * `cell_len` - The current cell length
	/// * `max_len` - The largest cell length
	fn calc_spacing(cell_len: u32, max_len: u32) -> u32 {
		max_len - cell_len + TABLE_COL_SPACING
	}

	/// Appends the specified amount of whitespaces to a string.
	fn fill_spacing(target: &mut String, spacing: u32) {
		for _ in 0..spacing {
			target.push_str(" ");
		}
	}

	/// Gets a value from a JSON object as a string.
	fn get_as_str(o: &JsonObject, key: &str) -> String {
		o.get(key).unwrap().to_string()
	}

	//
	// MAIN
	//

	let &CommandResult { data, fields } = result;

	let mut spacings: Vec<u32> = Vec::new();

	// Search for largest item in column.
	for f in fields {
		let mut spacing = f.len() as u32;

		for o in data {
			let str_len = get_as_str(o, f).len() as u32;

			if str_len > spacing {
				spacing = str_len;
			}
		}

		spacings.push(spacing);
	}

	let mut header = "".to_string();
	let mut divider = "".to_string();

	// Create the table header.
	for (i, f) in fields.iter().enumerate() {
		let f_len = f.len() as u32;
		header.push_str(f);

		for _ in 0..f_len {
			divider.push_str("-");
		}

		let max_space = *spacings.get(i).unwrap();
		let spacing = calc_spacing(f_len, max_space);

		if i != fields.len() - 1 {
			fill_spacing(&mut header, spacing);
			fill_spacing(&mut divider, spacing);
		}
	}

	let mut rows: Vec<String> = Vec::new();

	// Add the rows.
	for o in data {
		let mut row = "".to_string();

		for (i, f) in fields.iter().enumerate() {
			let data_str = get_as_str(o, f);

			let max_space = *spacings.get(i).unwrap();
			let spacing = calc_spacing(data_str.len() as u32, max_space);

			row.push_str(&*data_str);

			if i != fields.len() - 1 {
				fill_spacing(&mut row, spacing);
			}
		}

		rows.push(row);
	}

	format!("{}\n{}\n{}", header, divider, rows.join("\n"))
}

#[cfg(test)]
mod tests {
	use super::*;

	use serde_json::{json, Value};

	#[test]
	fn test_fmt_table() {
		let json = vec![
			json!({ "RequestId": 0, "NodeId": 12, "Data": { "key": "value" }, "Time": 0 })
				.as_object()
				.unwrap()
				.clone(),
			json!({ "RequestId": 1, "NodeId": 32, "Data": { "key": "value" }, "Time": 0 })
				.as_object()
				.unwrap()
				.clone(),
			json!({ "RequestId": 2, "NodeId": 353, "Data": { "key": "value" }, "Time": 0 })
				.as_object()
				.unwrap()
				.clone(),
		];

		let res = fmt_table(&CommandResult {
			fields: &vec!["RequestId", "NodeId", "Data", "Time"],
			data: &json,
		});

		/*
		Should look like this...

		RequestId        NodeId        Data                   Time
		---------        ------        ----                   ----
		0                12            {"key":"value"}        0
		1                32            {"key":"value"}        0
		2                353           {"key":"value"}        0
		*/

		let expected = "\
RequestId        NodeId        Data                   Time
---------        ------        ----                   ----
0                12            {\"key\":\"value\"}        0
1                32            {\"key\":\"value\"}        0
2                353           {\"key\":\"value\"}        0\
";

		println!("{}\n\n", expected);
		println!("{}", res);

		assert_eq!(res, expected);
	}
}
