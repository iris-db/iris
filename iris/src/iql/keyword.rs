use crate::database::graph::Graph;
use crate::iql::keywords::Insert;
use crate::lib::json::JsonObject;
use std::collections::HashMap;

/// Error while attempting to execute an action.
pub enum KeywordExecutionError {
	/// Keyword body is missing a JSON key.
	/// Example: `Missing key at security.credentials.username`
	MissingKey(String),
}

/// An IQL keyword.
pub trait Keyword {
	/// The key of the keyword.
	/// Example: `INSERT`
	fn key(&self) -> String;
	/// Implementation of the keyword.
	fn exec(&self) -> Result<(), KeywordExecutionError>;
}

impl dyn Keyword {
	/// Get all keywords.
	fn keywords<'a>() -> Vec<&'a dyn Keyword> {
		vec![&Insert]
	}
}

/// Holds the information for executing keywords.
pub struct DispatchKeywordContext<'a, 'b> {
	/// The graph to execute the keyword on.
	graph: Option<&'a mut Graph>,
	/// The JSON object holding the keywords.
	query_object: &'b JsonObject,
}

/// Dispatches all keyword actions from an object in order.
///
/// Returns the executed keywords.
pub fn dispatch_keywords(ctx: DispatchKeywordContext) -> HashMap<String, ()> {
	let DispatchKeywordContext {
		graph,
		query_object,
	} = ctx;

	HashMap::new()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::io::page::WriteError;

	#[test]
	pub fn test_dispatch_keywords() -> Result<(), WriteError> {
		let mut graph = Graph::new("a")?;
		let ctx = DispatchKeywordContext {
			graph: Some(&mut graph),
			query_object: &serde_json::from_str(
				r#"
			[
				{
					"INSERT": {}
				},
				{
					"DELETE": {}
				}
			]
			"#,
			)
			.unwrap(),
		};

		let res = dispatch_keywords(ctx);

		let keys = res.keys();

		Ok(())
	}
}
