use std::sync::Mutex;

use rocket::config::{Environment, LoggingLevel};
use rocket::{Config, State};
use rocket_contrib::json::Json;
use serde_json::{json, Value};

use crate::graph::database::Database;
use crate::graph::graph::Graph;
use crate::lib::bson::JsonObject;
use crate::query::directive::new_error_object;
use crate::server::http::context::HttpContext;

type RouteContext = Mutex<Database>;

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
    .mount("/", routes![dispatch_query])
    .manage(Mutex::new(db))
    .launch();
}

#[post("/graphs/<graph_name>", data = "<body>")]
fn dispatch_query(
  graph_name: String,
  body: Json<JsonObject>,
  ctx: State<RouteContext>,
) -> Json<Value> {
  let mut db = ctx.inner().lock().unwrap();
  let (graphs, directives) = db.route_ctx();

  let mut results: Vec<Value> = Vec::new();
  let mut errors: Vec<Value> = Vec::new();

  let graph: Option<&mut Box<Graph>> = graphs.get_mut(graph_name.as_str());

  let graph = match graph {
    Some(v) => v,
    None => {
      errors.push(json!({
        "error": format!("Graph {} does not exist", graph_name)
      }));
      return construct_result_object(&results, &errors);
    }
  };

  let data = body.0;

  for k in data.keys() {
    let directive = directives.get(k);
    let directive = match directive {
      Some(v) => v,
      None => continue,
    };

    let ctx = HttpContext::try_new(graph, *directive, &data);
    let ctx = match ctx {
      Ok(ctx) => ctx,
      Err(e) => {
        errors.push(Value::Object(new_error_object(k, e)));
        continue;
      }
    };

    let res = directive.exec(ctx);
    let mut res = match res {
      Ok(v) => v,
      Err(v) => {
        errors.push(new_error_object(k, v).into());
        continue;
      }
    };

    let mut final_result: JsonObject = JsonObject::new();
    final_result.insert(
      "directive".to_string(),
      Value::String(directive.key().to_string()),
    );

    final_result.append(&mut res);
    results.push(Value::Object(final_result));
  }

  construct_result_object(&results, &errors)
}

fn construct_result_object(results: &Vec<Value>, errors: &Vec<Value>) -> Json<Value> {
  Json(json!({
    "results": results,
    "errors": errors,
  }))
}
