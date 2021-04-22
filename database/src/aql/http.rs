use std::sync::Mutex;

use rocket::{Config, State};
use rocket_contrib::json::Json;
use serde_json::json;

use crate::aql::directive::DIRECTIVE_PREFIX;
use crate::graph::database::Database;
use crate::graph::graph::Graph;
use crate::lib::bson::JsonObject;
use rocket::config::{Environment, LoggingLevel};

/// Rocket route context.
struct RouteContext {
    db: Mutex<Database>,
}

/// Starts the rest server.
pub fn start_rest_server() {
    let db = Database::new();

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
) -> Json<JsonObject> {
    let db = ctx.db.lock().unwrap();

    let graphs = db.graphs();
    let directives = db.directives();

    let mut graph: Option<&Box<Graph>> = None;

    for g in graphs {
        if g.name().eq(&graph_name) {
            graph = Some(g);
        }
    }

    let graph = match graph {
        Some(v) => v,
        None => {
            return Json(
                json!({ "error": format!("Graph {} does not exist", graph_name) })
                    .as_object()
                    .unwrap()
                    .clone(),
            )
        }
    };

    let data = body.0;
    for k in data.keys() {
        let index =
            directives.binary_search_by(|d| format!("{}{}", DIRECTIVE_PREFIX, d.key()).cmp(k));
        let index = match index {
            Ok(v) => v,
            Err(_) => continue,
        };

        let directive = &directives[index];
        // let res = directive.exec(plane);
    }

    Json(
        json!({ "completedOnGraph": graph_name })
            .as_object()
            .unwrap()
            .clone(),
    )
}
