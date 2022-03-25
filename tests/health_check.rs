//! tests/health_check.rs

#[tokio::test]
async fn health_check_works() {
    // arrange
    spawn_app();
    // http client library
    let client = reqwest::Client::new();

    // Action
    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = rust_0_2_prod::run().expect("Failed to bind server");
    let _ = tokio::spawn(server);
}
