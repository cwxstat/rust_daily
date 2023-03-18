use hyper::StatusCode;
use hyper::{Body, Request, Response};
use std::convert::Infallible;

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

pub async fn handle_request(
    spitter: impl Spitter,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
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
