mod integration_common;

use integration_common::{build_client, build_client_with_key};

#[tokio::test]
#[ignore]
async fn ping_should_succeed() {
  let client = build_client();
  client.ping().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn ping_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.ping().await;
  assert!(result.is_err());
}
