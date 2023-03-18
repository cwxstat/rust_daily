use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use std::convert::Infallible;
use std::net::SocketAddr;
use web_server::handle_request;
use web_server::RealSpitter;

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
