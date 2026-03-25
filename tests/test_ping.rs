mod common;

use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_ping_success() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Ping"))
    .and(header("x-apikey", "test-api-key"))
    .and(header("x-api-version", "2"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

  let result = client.ping().await;
  assert!(result.is_ok());
}

#[tokio::test]
async fn test_ping_unauthorized() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Ping"))
    .respond_with(ResponseTemplate::new(401))
    .mount(&mock_server)
    .await;

  let result = client.ping().await;
  assert!(result.is_err());

  if let Err(onspring::OnspringError::Api { status_code, .. }) = result {
    assert_eq!(status_code, 401);
  } else {
    panic!("Expected Api error");
  }
}
