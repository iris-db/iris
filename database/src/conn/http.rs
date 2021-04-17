use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

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
