use std::collections::HashMap;
use std::sync::Mutex;

use rocket::config::{Environment, LoggingLevel};
use rocket::http::Status;
use rocket::{Config, Request, Response, State};
use rocket_contrib::json::Json;

use crate::conn::response_builder;
use crate::conn::response_builder::ResponseFormat::JSON;
use crate::conn::response_builder::{ResponseFormat, DEFAULT_RESPONSE_FORMAT};
use crate::database::graph::Graph;
use crate::io::logger;
use crate::io::logger::s_log;
use crate::io::logger::EventCategory::Network;
use crate::io::logger::EventSeverity::Info;
use crate::iql::keyword::Keyword;
use crate::lib::json::{fmt_table, JsonObject};
use rocket::request::{FromRequest, Outcome};
use serde_json::Value;

/// An IrisDB server.
pub struct Server {
	max_cache_size: usize,
	port: u16,
}

impl Server {
	pub fn new(port: u16, max_cache_size: usize) -> Self {
		Self {
			port,
			max_cache_size,
		}
	}

	/// Starts the HTTP server.
	pub fn start(&self) {
		let config = Config::build(Environment::Staging)
			.port(self.port)
			.log_level(LoggingLevel::Off)
			.finalize()
			.unwrap();

		s_log(
			Info,
			Network,
			&*format!("HTTP protocol started on http://localhost:{}", self.port),
		);

		rocket::custom(config)
			.mount("/", routes![graph_query])
			.manage(Mutex::new(Database::new()))
			.launch();
	}
}

/// An in memory representation of the database.
pub struct Database {
	graphs: HashMap<String, Graph>,
}

impl Database {
	fn new() -> Self {
		Self {
			graphs: HashMap::new(),
		}
	}

	/// Calculates the total amount of memory being used by the internal database.
	fn mem_size() -> u32 {
		0
	}

	fn graphs(&mut self) -> &mut HashMap<String, Graph> {
		&mut self.graphs
	}
}

#[post("/graph/_query", data = "<body>")]
fn graph_query(
	body: Json<JsonObject>,
	ctx: State<Mutex<Database>>,
	rf: ResponseFormat,
) -> Response {
	let body = Value::from(body.into_inner());
	let mut db = ctx.inner().lock().unwrap();

	let query = &body["Query"];
	if query.is_null() || !query.is_array() {
		let res = fmt_table(&vec![response_builder::json_error_object(
			"Expected an array for Query",
			&JsonObject::new(),
		)]);
		return response_builder::new_response(rf, Status::Ok, res);
	}

	response_builder::new_response(JSON, Status::Ok, "{}\n".to_string())
}
