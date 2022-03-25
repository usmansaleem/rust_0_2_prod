//! tests/health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let url = spawn_app();
    // http client library
    let client = reqwest::Client::new();

    // Action
    let response = client
        .get(&format!("{}/health_check", &url))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = rust_0_2_prod::run(listener).expect("Failed to bind server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
