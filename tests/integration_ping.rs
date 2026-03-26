mod integration_common;

use integration_common::{build_client, build_client_with_key};

#[tokio::test]
#[ignore]
async fn ping_should_succeed_with_api_key() {
  let client = build_client();
  client.ping().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn ping_should_succeed_without_api_key() {
  let client = build_client_with_key("");
  client.ping().await.unwrap();
}
