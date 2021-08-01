use hyper::http::uri::Authority;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let new_host = std::env::var("NEW_HOST").expect("NEW_HOST not specified");
    let original_location = req.uri();
    let location = hyper::Uri::builder()
        .scheme("https")
        .authority(Authority::from_str(new_host.as_ref()).expect("Invalid NEW_HOST"))
        .path_and_query(
            original_location
                .path_and_query()
                .expect("Invalid request")
                .clone(),
        )
        .build()
        .expect("Invalid uri");
    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", location.to_string())
        .body(Body::empty())
        .unwrap())
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
