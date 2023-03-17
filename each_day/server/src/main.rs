use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;
use std::convert::Infallible;
use std::net::SocketAddr;

#[cfg_attr(test, mockall::automock)]
pub trait Spitter {
    fn spit(&self) -> String;
    fn spit2(&self) -> String;
}

#[derive(Clone)]
pub struct RealSpitter;

impl Spitter for RealSpitter {
    fn spit(&self) -> String {
        "Hello, I'm the return string from the spit function!".to_string()
    }

    fn spit2(&self) -> String {
        "Hello, I'm the return string from the spit2 function!".to_string()
    }
}

async fn handle_request(spitter: impl Spitter, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (parts, _body) = req.into_parts();
    let message = match parts.uri.path() {
        "/split222" => spitter.spit2(),
        _ => spitter.spit(),
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(message))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let spitter = RealSpitter;
    let make_svc = make_service_fn(move |_conn| {
        let spitter = spitter.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let spitter = spitter.clone();
                handle_request(spitter, req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
