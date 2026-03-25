use onspring::OnspringClient;
use wiremock::MockServer;

pub async fn setup() -> (MockServer, OnspringClient) {
  let mock_server = MockServer::start().await;
  let client = OnspringClient::builder("test-api-key")
    .base_url(mock_server.uri())
    .build();
  (mock_server, client)
}
