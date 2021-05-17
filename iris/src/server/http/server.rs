use std::sync::{Mutex, MutexGuard};

use rocket::config::{Environment, LoggingLevel};
use rocket::{Config, State};
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::graph::database::Database;
use crate::graph::graph::Graph;
use crate::server::http::errors;

pub type RouteContext<'a> = State<'a, Mutex<Database>>;

/// Starts the REST api.
pub fn start() {
	let db = Database::new();

	println!(
		"\
Iris
======================================
PORT: 6000
ClusterConnections: 1"
	);

	let config = Config::build(Environment::Staging)
		.port(6000)
		.log_level(LoggingLevel::Off)
		.finalize()
		.unwrap();

	rocket::custom(config)
		.mount("/", routes![])
		.manage(Mutex::new(db))
		.launch();
}

pub fn extract_graph<'a>(
	mut db: MutexGuard<'a, Database>,
	graph_name: &str,
) -> Result<&'a mut Box<Graph>, Json<Value>> {
	let graphs = db.graphs();

	let graph = graphs.get_mut(graph_name);

	return match graph {
		Some(graph) => Ok(graph),
		None => Err(errors::graph_not_found(graph_name)),
	};
}
