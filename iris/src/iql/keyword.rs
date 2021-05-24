use crate::database::graph::Graph;
use crate::iql::keywords::Insert;
use crate::lib::json::JsonObject;
use serde_json::Value;
use std::collections::HashMap;

/// Error while attempting to execute an action.
pub enum KeywordExecutionError {
	/// Keyword body is missing a JSON key.
	///
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

/// Holds the information for a query (multiple keywords in a sequence).
pub struct DispatchQueryContext<'a, 'b> {
	/// The graph to execute the keyword on.
	graph: Option<&'a mut Graph>,
	/// The JSON array holding the query keywords.
	query: &'b Vec<Value>,
	/// The object that must be returned from the query.
	return_stmt: &'b str,
}

impl<'a, 'b> DispatchQueryContext<'a, 'b> {
	pub fn new(graph: Option<&'a mut Graph>, query: &'b Vec<Value>, return_stmt: &'b str) -> Self {
		Self {
			graph,
			query,
			return_stmt,
		}
	}

	pub fn execute_query() {}
}

/// Dispatches all keyword actions from an object in order.
///
/// Returns the executed keywords.
pub fn dispatch_keywords(ctx: DispatchQueryContext) -> HashMap<String, ()> {
	let DispatchQueryContext {
		graph,
		query,
		return_stmt,
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
		let ctx = DispatchQueryContext {
			graph: Some(&mut graph),
			query: &serde_json::from_str(
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
			return_stmt: "*",
		};

		let res = dispatch_keywords(ctx);

		let keys = res.keys();

		Ok(())
	}
}
