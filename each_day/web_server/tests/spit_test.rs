// tests/web_server_tests.rs
use hyper::{Body, Request, StatusCode};
use tokio::runtime::Runtime;
use web_server::{handle_request, Spitter, MockSpitter};

use mockall::predicate::*;
use mockall::Sequence;


#[test]
fn test_handle_request() {
    let mut runtime = Runtime::new().unwrap();

    let mut spitter_mock = web_server::MockSpitter::new();
    let mut seq = Sequence::new();
    spitter_mock
        .expect_spit()
        .times(1)
        .in_sequence(&mut seq)
        .return_const("Test spit response".to_string());
    spitter_mock
        .expect_spit2()
        .times(1)
        .in_sequence(&mut seq)
        .return_const("Test spit2 response".to_string());

    let test_cases = vec![
        ("/", "Test spit response"),
        ("/split222", "Test spit2 response"),
    ];

    for (path, expected_response) in test_cases {
        let request = Request::get(format!("http://localhost:3000{}", path))
            .body(Body::empty())
            .unwrap();

        let response = runtime.block_on(handle_request(spitter_mock.clone(), request)).unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = runtime.block_on(hyper::body::to_bytes(response.into_body())).unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        assert_eq!(body_str, expected_response);
    }
}
