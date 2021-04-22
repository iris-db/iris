use std::sync::Mutex;

use rocket::{Config, State};
use rocket_contrib::json::Json;
use serde_json::json;

use crate::aql::directive::DIRECTIVE_PREFIX;
use crate::graph::database::Database;
use crate::graph::node_plane::NodePlane;
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

#[post("/<plane_name>", data = "<body>")]
fn dispatch_query(
    plane_name: String,
    body: Json<JsonObject>,
    ctx: State<RouteContext>,
) -> Json<JsonObject> {
    let db = ctx.db.lock().unwrap();

    let planes = db.planes();
    let directives = db.directives();

    let mut plane: Option<&Box<NodePlane>> = None;

    for p in planes {
        if p.name().eq(&plane_name) {
            plane = Some(p);
        }
    }

    let plane = match plane {
        Some(v) => v,
        None => {
            return Json(
                json!({ "error": format!("Plane {} does not exist", plane_name) })
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
        let res = directive.exec(plane);
    }

    Json(
        json!({ "completedOnPlane": plane_name })
            .as_object()
            .unwrap()
            .clone(),
    )
}
