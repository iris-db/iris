use rocket_contrib::json::Json;
use serde_json::{json, Value};

use crate::lib::bson::JsonObject;
use crate::lib::smart_json::{KeyNotFoundError, SmartJsonObject};
use crate::server::http::response_builder;
use crate::server::http::server::{extract_graph, RouteContext};
use rocket::http::{ContentType, Status};
use rocket::Response;

#[post("/engine/graph", data = "<body>")]
pub fn query(body: Json<JsonObject>, ctx: RouteContext) -> Response {
	let mut ctx = ctx.inner().lock().unwrap();

	let data = SmartJsonObject(&body.0);

	// Default ContentType
	let mut content_type: ContentType = ContentType::JSON;

	let res_fmt = data.get_owned("format");
	let res_fmt = match res_fmt {
		Ok(name) => name,
		Err(e) => return response_builder::from(content_type, e),
	};

	let graph_name = data.get_owned("graph");
	let graph_name = match graph_name {
		Ok(name) => name,
		Err(e) => return response_builder::from(ContentType::Plain, e),
	};

	if !graph_name.is_string() {
		return response_builder::new_response();
	}

	let graph = extract_graph(ctx, &*name);
	let graph = match graph {
		Ok(graph) => graph,
		Err(e) => return e,
	};

	let data = "";

	response_builder::new_response(ContentType::Plain, data.to_string(), Status::Ok)
}
