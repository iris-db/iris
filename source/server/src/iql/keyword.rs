use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::iql::keywords::*;
use crate::lib::json::types::JsonObject;
use crate::storage_engines::affinity::graph::Graph;

/// Get all keywords.
///
/// Should only be called once at the start of the program.
pub fn get_registered_keywords() -> KeywordMap {
    KeywordRegistration(vec![Box::new(Insert)]).into()
}

/// An IQL keyword.
pub trait Keyword: Send {
    /// The JSON key of the keyword.
    /// Example: `INSERT`
    fn key(&self) -> String;
    /// Implementation of the keyword.
    fn exec(&self, ctx: DispatchKeywordContext) -> Result<JsonObject, KeywordError>;
}

/// Maps a `Keyword`'s key name to its implementation.
type KeywordMap = HashMap<String, Box<dyn Keyword>>;

/// Wrapper type for converting to a `KeywordMap`.
pub struct KeywordRegistration(pub Vec<Box<dyn Keyword>>);

impl From<KeywordRegistration> for KeywordMap {
    fn from(kw_reg: KeywordRegistration) -> Self {
        let mut m = KeywordMap::new();

        for kw in kw_reg.0 {
            m.insert(kw.key(), kw);
        }

        m
    }
}

/// Holds the data required for a `Keyword` to mutate the database.
pub struct DispatchKeywordContext<'a> {
    graph: &'a Option<&'a mut Graph>,
    data: &'a Value,
}

impl<'a> DispatchKeywordContext<'a> {
    fn new(graph: &'a Option<&mut Graph>, data: &'a Value) -> Self {
        Self { graph, data }
    }

    pub fn graph(&self) -> &&Option<&mut Graph> {
        &self.graph
    }

    pub fn data<T>(&self) -> Result<T, serde_json::Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        serde_json::from_value(self.data.clone())
    }
}

/// Holds the information for a query (multiple keywords in a sequence).
pub struct DispatchQueryContext<'a, 'b, 'c> {
    /// The graph to execute the keyword on.
    graph: Option<&'a mut Graph>,
    /// The JSON array holding the query keywords.
    query: &'b Vec<Value>,
    /// The object that must be returned from the query.
    return_stmt: &'b str,
    /// A reference to the registered keywords.
    keywords: &'c KeywordMap,
}

#[derive(Serialize, Deserialize)]
/// A successful `Keyword` execution.
pub struct KeywordResult {
    /// The keyword performed.
    pub keyword: Option<String>,
    #[serde(rename = "ref")]
    /// The ref of the query result.
    pub ref_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
/// A unsuccessful `Keyword` execution.
pub struct KeywordError {
    /// The keyword performed.
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
    results: Vec<KeywordResult>,
    /// Failed query execution error.
    errors: Vec<KeywordError>,
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
    pub fn result(&mut self, result: KeywordResult) -> &mut QueryExecutionResult {
        self.results.push(result);
        self
    }

    /// Adds an error to the query result.
    pub fn error(&mut self, error: KeywordError) -> &mut QueryExecutionResult {
        self.errors.push(error);
        self
    }
}

impl<'a, 'b, 'c> DispatchQueryContext<'a, 'b, 'c> {
    pub fn new(
        graph: Option<&'a mut Graph>,
        query: &'b Vec<Value>,
        return_stmt: &'b str,
        keywords: &'c KeywordMap,
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
                res.error(KeywordError {
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
                    res.error(KeywordError {
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
