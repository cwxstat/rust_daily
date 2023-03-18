use hyper::{Body, Request, Response, Server};
use mockall::{automock, predicate::*};
use web_server::handle_request;
use web_server::RealSpitter;
use web_server::Spitter;

#[automock]
trait MockSpitter: Spitter {}

#[tokio::test]
async fn test_handle_request_with_mocked_spitter() {
    let mut mock_spitter = MockSpitter::new();
    mock_spitter
        .expect_spit()
        .times(1)
        .return_const("Mocked spit response".to_string());
    mock_spitter
        .expect_spit2()
        .times(1)
        .return_const("Mocked spit2 response".to_string());

    let request1 = Request::get("http://127.0.0.1:8080/some_path")
        .body(Body::empty())
        .unwrap();

    let response1 = handle_request(mock_spitter.clone(), request1)
        .await
        .unwrap();
    assert_eq!(response1.into_body(), Body::from("Mocked spit response"));

    let request2 = Request::get("http://127.0.0.1:8080/split222")
        .body(Body::empty())
        .unwrap();

    let response2 = handle_request(mock_spitter, request2).await.unwrap();
    assert_eq!(response2.into_body(), Body::from("Mocked spit2 response"));
}
