use std::convert::{Infallible, TryFrom};
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rocket_contrib::json::Json;
use serde_json::json;

use crate::lib::bson::JsonObject;

async fn aql_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let s = full_body.iter().cloned().collect::<Vec<u8>>();

    println!("{}", String::from_utf8(s).unwrap());

    Ok(Response::new("Hi".into()))
}

/// Database entrypoint. An HTTP server that accepts a POST request with a plaintext body
/// containing the AQL query string.
pub async fn start_http_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 6123));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(aql_handler)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("HTTP SERVER ERROR: {}", e);
    }
}

#[post("/<plane>")]
pub fn mutate_plane(plane: String) -> Json<JsonObject> {
    Json(json!({ "hello": plane }).as_object().unwrap().clone())
}

pub fn start_rest_server() {
    rocket::ignite().mount("/", routes![mutate_plane]).launch();
}
