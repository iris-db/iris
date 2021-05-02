use std::sync::Mutex;

use rocket::config::{Environment, LoggingLevel};
use rocket::{Config, State};
use rocket_contrib::json::Json;
use serde_json::{json, Value};

use crate::graph::database::Database;
use crate::graph::node::Node;
use crate::lib::bson::JsonObject;
use crate::server::http::errors;

type RouteContext<'a> = State<'a, Mutex<Database>>;

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
    .mount("/", routes![insert_node])
    .manage(Mutex::new(db))
    .launch();
}

#[post("/graphs/<name>/_node", data = "<body>")]
fn insert_node(name: String, body: Json<Vec<JsonObject>>, ctx: RouteContext) -> Json<Value> {
  let mut ctx = ctx.inner().lock().unwrap();

  let graphs = ctx.graphs();

  let graph = graphs.get_mut(&*name);

  let mut total_time: u64 = 0;

  let graph = match graph {
    Some(graph) => graph,
    None => return errors::graph_not_found(name),
  };

  let data = body.0;
  for req in data {
    let id = graph.next_id();

    let data = req.get("data");
    let data = match data {
      Some(value) => Some(value.clone()),
      None => None,
    };

    // let insert_time = graph.insert(Node::new(id, data, None))?;
    // total_time += insert_time;
  }

  let count = data.len();

  Json(json!({
    "time": total_time,
    "count": count
  }))
}
