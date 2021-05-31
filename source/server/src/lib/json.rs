use serde_json::{Map, Value};

use crate::conn::response_builder::ResponseFormat;

pub type JsonObject = Map<String, Value>;

/// Represents a a raw BSON document.
pub struct RawBson<'a>(&'a Vec<u8>);

impl RawBson<'_> {
	/// Converts the raw BSON into a human readable string based on a ResponseFormat.
	pub fn marshal(&self, fmt: ResponseFormat) -> String {
		return match fmt {
			ResponseFormat::Table => "".to_string(),
			ResponseFormat::JSON => "".to_string(),
		};
	}
}

/// The amount of spaces used to separate a table column.
const TABLE_COL_SPACING: u32 = 8;

/// Formats a JSON object as a table.
pub fn fmt_table(result: &Vec<JsonObject>) -> String {
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

	let mut fields: Vec<&String> = Vec::new();
	let mut data: Vec<JsonObject> = Vec::new();

	for r in result {
		for k in r.keys() {
			if !fields.contains(&k) {
				fields.push(k);
			}
		}

		data.push(r.clone());
	}

	let mut spacings: Vec<u32> = Vec::new();

	// Search for largest item in column.
	for f in &fields {
		let mut spacing = f.len() as u32;

		for o in &data {
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
			let data_str = get_as_str(&o, f);

			let max_space = *spacings.get(i).unwrap();
			let spacing = calc_spacing(data_str.len() as u32, max_space);

			row.push_str(&*data_str);

			if i != fields.len() - 1 {
				fill_spacing(&mut row, spacing);
			}
		}

		rows.push(row);
	}

	format!("{}\n{}\n{}\n", header, divider, rows.join("\n"))
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

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

		let res = fmt_table(&json);

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
2                353           {\"key\":\"value\"}        0
";

		assert_eq!(res, expected);
	}
}
