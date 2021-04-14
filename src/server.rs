#![feature(option_insert)]
#![feature(map_first_last)]

use std::convert::Infallible;
use std::net::SocketAddr;

use crate::graph::node_plane::NodePlane;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::fs;
use std::ops::Add;

mod graph;
mod io;
mod lib;

#[tokio::main]
async fn main() {
    // Prepare the filesystem by creating the required directories.
    let paths = ["Data", "Temp"];

    paths.iter().for_each(|p| {
        let res = fs::create_dir_all("Affinity/".to_owned().add(p));
        res.err().and_then(|e| -> Option<()> {
            panic!("{}", e.to_string());
        });
    });

    // Prepare and start the HTTP server.
    let addr = SocketAddr::from(([127, 0, 0, 1], 6123));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(aql_handler)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started on http://localhost:4000");
    if let Err(e) = server.await {
        eprintln!("HTTP SERVER ERROR: {}", e);
    }
}

/// Dispatches the AQL statement body.
async fn aql_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let s = full_body.iter().cloned().collect::<Vec<u8>>();

    let strbody = String::from_utf8(s).unwrap();

    let mut p = NodePlane::new("myn");
    let n = p.insert_node(None, serde_json::from_str(strbody.as_str()).unwrap());

    Ok(Response::new("Hi".into()))
}
