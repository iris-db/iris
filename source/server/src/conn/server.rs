use std::sync::Mutex;

use rocket::config::{Environment, LoggingLevel};
use rocket::http::Status;
use rocket::{Config, Response, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::conn::response_builder;
use crate::conn::response_builder::ResponseFormat;
use crate::conn::response_builder::ResponseFormat::JSON;
use crate::io::logger::s_log;
use crate::io::logger::EventCategory::Network;
use crate::io::logger::EventSeverity::Info;
use crate::lib::json::types::{JsonObject, SmartJson};
use crate::storage_engines::orion::database::Database;

/// An IrisDB server.
pub struct Server {
    /// Max cache size for internal cache engine.
    max_cache_size: usize,
    /// Exposed port for the HTTP server.
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

#[derive(Serialize, Deserialize)]
struct RequestBody {
    graph: Option<String>,
    query: Option<Vec<Value>>,
    #[serde(rename = "return")]
    return_stmt: Option<String>,
}

#[post("/graph/_query", data = "<body>")]
fn graph_query<'a>(
    body: Json<JsonObject>,
    ctx: State<Mutex<Database>>,
    rf: ResponseFormat,
) -> Response<'a> {
    let mut db = ctx.inner().lock().unwrap();

    let body = Value::from(body.into_inner());

    let body: RequestBody = SmartJson::from(body).into_struct().ok().unwrap();

    response_builder::new_response(JSON, Status::Ok, "{}\n".to_string())
}
