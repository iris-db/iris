use rocket_contrib::json::Json;
use serde_json::{json, Value};

pub fn graph_not_found(name: String) -> Json<Value> {
  Json(json!({}))
}
