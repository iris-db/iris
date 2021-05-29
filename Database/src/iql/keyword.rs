use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::database::graph::Graph;
use crate::iql::keywords::{Delete, Insert};
use crate::lib::json::JsonSerializable;
use std::any::Any;

pub struct DispatchKeywordContext<'a, T>
where
	T: JsonSerializable,
{
	graph: &'a Option<&'a mut Graph>,
	data: &'a T,
}

impl<'a, T> DispatchKeywordContext<'a, T>
where
	T: JsonSerializable,
{
	fn new(graph: &'a Option<&mut Graph>, data: &'a T) -> Self {
		Self { graph, data }
	}

	pub fn graph(&self) -> &&Option<&mut Graph> {
		&self.graph
	}

	pub fn data(&self) -> &&T {
		&self.data
	}
}

/// An IQL keyword.
pub trait Keyword: Send {
	type Args: JsonSerializable;
	type Ok: JsonSerializable;

	/// The key of the keyword.
	/// Example: `INSERT`
	fn key(&self) -> String;
	/// Implementation of the keyword.
	fn exec(&self, ctx: DispatchKeywordContext<Self::Args>) -> Result<Self::Ok, QueryError>;
}

/// Get all keywords.
///
/// Should only be called once at the start of the program.
pub fn get_registered_keywords<'a, A: Keyword, B: JsonSerializable>(x: &str) -> SizedKeyword<A, B> {
	return match x {
		_ => Box::new(Insert),
	};
}

pub type SizedKeyword<A, B> = Box<dyn Keyword<Ok = A, Args = B>>;

/// A HashMap mapping a String (the keyword JSON key) to the keyword implementation.
pub type KeywordMap<A, B> = HashMap<String, SizedKeyword<A, B>>;

/// A list of all registered Iris Query Language keywords.
struct KeywordRegistration<A, B>(pub Vec<SizedKeyword<A, B>>);

impl<A, B> From<KeywordRegistration<A, B>> for KeywordMap<A, B> {
	fn from(kw_reg: KeywordRegistration<A, B>) -> Self {
		let mut map = KeywordMap::new();

		for kw in kw_reg.0 {
			map.insert(kw.key(), kw);
		}

		map
	}
}

/// Holds the information for a query (multiple keywords in a sequence).
pub struct DispatchQueryContext<'a, 'b, 'c, A, B> {
	/// The graph to execute the keyword on.
	graph: Option<&'a mut Graph>,
	/// The JSON array holding the query keywords.
	query: &'b Vec<Value>,
	/// The object that must be returned from the query.
	return_stmt: &'b str,
	/// A reference to the registered keywords.
	keywords: &'c KeywordMap<A, B>,
}

#[derive(Serialize, Deserialize)]
/// A successful query result.
pub struct QueryResult {
	/// The query action performed.
	pub keyword: Option<String>,
	#[serde(rename = "ref")]
	/// The ref of the query result.
	pub ref_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryError {
	/// The query action performed.
	pub keyword: Option<String>,
	/// Error message.
	pub msg: Option<String>,
	/// Error data.
	pub data: Option<Value>,
}

#[derive(Serialize, Deserialize)]
/// Result of executing a query.
pub struct QueryExecutionResult {
	/// Successfully executed query return data.
	results: Vec<QueryResult>,
	/// Failed query execution error.
	errors: Vec<QueryError>,
}

impl QueryExecutionResult {
	/// Creates a new `QueryExecutionResult`, holding zero data and errors.
	pub fn new() -> Self {
		Self {
			results: Vec::new(),
			errors: Vec::new(),
		}
	}

	/// Adds data to the query result.
	pub fn result(&mut self, result: QueryResult) -> &mut QueryExecutionResult {
		self.results.push(result);
		self
	}

	/// Adds an error to the query result.
	pub fn error(&mut self, error: QueryError) -> &mut QueryExecutionResult {
		self.errors.push(error);
		self
	}
}

impl<'a, 'b, 'c, A, B> DispatchQueryContext<'a, 'b, 'c, A, B> {
	pub fn new(
		graph: Option<&'a mut Graph>,
		query: &'b Vec<Value>,
		return_stmt: &'b str,
		keywords: &'c KeywordMap<A, B>,
	) -> Self {
		DispatchQueryContext {
			graph,
			query,
			return_stmt,
			keywords,
		}
	}

	/// Executes the IQL query.
	pub fn execute(&self) -> QueryExecutionResult {
		let mut res = QueryExecutionResult::new();

		for (i, query_stmt) in self.query.iter().enumerate() {
			let stmt = &query_stmt[0];
			if !stmt.is_object() {
				res.error(QueryError {
					keyword: None,
					msg: "Expected an object".to_string().into(),
					data: json!({ "queryIndex": i }).into(),
				});
			}

			let stmt = stmt.as_object().unwrap();
			let query_key = stmt.keys().into_iter().next().unwrap();
			let query_data = &stmt[query_key];

			let kw = self.keywords.get(query_key);
			let kw = match kw {
				Some(kw) => kw,
				None => {
					res.error(QueryError {
						keyword: query_key.clone().into(),
						msg: format!("Unknown query directive {}", query_key).into(),
						data: json!({ "keyword": query_key }).into(),
					});
					continue;
				}
			};

			let kw_exec_res = kw.exec(DispatchKeywordContext::new(&self.graph, query_data));
			let kw_exec_res = match kw_exec_res {
				Ok(v) => v,
				Err(e) => {
					res.error(e);
					continue;
				}
			};
		}

		res
	}
}
