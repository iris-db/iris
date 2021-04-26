use std::borrow::BorrowMut;
use std::sync::Mutex;

use rocket::config::{Environment, LoggingLevel};
use rocket::{Config, State};
use rocket_contrib::json::Json;
use serde_json::{json, Value};

use crate::aql::context::AqlContext;
use crate::aql::directive::{DirectiveError, DIRECTIVE_PREFIX};
use crate::graph::database::Database;
use crate::graph::graph::Graph;
use crate::lib::bson::JsonObject;

/// Rocket route context.
struct RouteContext {
  db: Mutex<Database>,
}

/// Starts the rest server.
pub fn start_rest_server() {
  let db = Database::new();

  println!(
    "\
CallistoDB
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
    .mount("/", routes![dispatch_query])
    .manage(RouteContext { db: Mutex::new(db) })
    .launch();
}

#[post("/graphs/<graph_name>", data = "<body>")]
fn dispatch_query(
  graph_name: String,
  body: Json<JsonObject>,
  ctx: State<RouteContext>,
) -> Json<Vec<Value>> {
  let mut db = ctx.db.lock().unwrap();

  let (graphs, directives) = db.ctx_data();

  let graph: Option<&mut Box<Graph>> = graphs.get_mut(graph_name.as_str());

  let graph = match graph {
    Some(v) => v,
    None => {
      return Json(vec![json!({
        "error": format!("Graph {} does not exist", graph_name)
      })]);
    }
  };

  let data = body.0;

  let mut ctx = AqlContext::new(graph, &data);

  let mut directive_results: Vec<Value> = Vec::new();

  for k in data.keys() {
    let index = directives.binary_search_by(|d| format!("{}{}", DIRECTIVE_PREFIX, d.key()).cmp(k));
    let index = match index {
      Ok(v) => v,
      Err(_) => continue,
    };

    let directive = &directives[index];

    let res = directive.exec(&mut ctx);
    let mut res = match res {
      Ok(v) => v,
      Err(v) => DirectiveError {
        directive_key: k,
        err_type: v,
      }
      .into(),
    };

    let mut final_result: JsonObject = JsonObject::new();
    final_result.insert(
      "directive".to_string(),
      Value::String(directive.key().to_string()),
    );
    final_result.append(&mut res);

    directive_results.push(Value::Object(final_result));
  }

  Json(directive_results)
}
