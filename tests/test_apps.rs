mod common;

use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_list_apps_success() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Apps"))
    .and(header("x-apikey", "test-api-key"))
    .and(header("x-api-version", "2"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "pageNumber": 1,
        "pageSize": 50,
        "totalPages": 1,
        "totalRecords": 2,
        "items": [
            {"href": "/Apps/id/1", "id": 1, "name": "App One"},
            {"href": "/Apps/id/2", "id": 2, "name": "App Two"}
        ]
    })))
    .mount(&mock_server)
    .await;

  let result = client.list_apps(None).await.unwrap();
  assert_eq!(result.total_records, Some(2));
  let items = result.items.unwrap();
  assert_eq!(items.len(), 2);
  assert_eq!(items[0].id, 1);
  assert_eq!(items[0].name.as_deref(), Some("App One"));
}

#[tokio::test]
async fn test_list_apps_with_paging() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Apps"))
    .and(query_param("PageNumber", "2"))
    .and(query_param("PageSize", "10"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "pageNumber": 2,
        "pageSize": 10,
        "totalPages": 3,
        "totalRecords": 25,
        "items": []
    })))
    .mount(&mock_server)
    .await;

  let paging = onspring::PagingRequest {
    page_number: 2,
    page_size: 10,
  };
  let result = client.list_apps(Some(paging)).await.unwrap();
  assert_eq!(result.page_number, Some(2));
}

#[tokio::test]
async fn test_get_app_success() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Apps/id/42"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "href": "/Apps/id/42",
        "id": 42,
        "name": "Test App"
    })))
    .mount(&mock_server)
    .await;

  let app = client.get_app(42).await.unwrap();
  assert_eq!(app.id, 42);
  assert_eq!(app.name.as_deref(), Some("Test App"));
}

#[tokio::test]
async fn test_get_app_not_found() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Apps/id/999"))
    .respond_with(ResponseTemplate::new(404))
    .mount(&mock_server)
    .await;

  let result = client.get_app(999).await;
  assert!(result.is_err());
  if let Err(onspring::OnspringError::Api { status_code, .. }) = result {
    assert_eq!(status_code, 404);
  } else {
    panic!("Expected Api error");
  }
}

#[tokio::test]
async fn test_batch_get_apps() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("POST"))
    .and(path("/Apps/batch-get"))
    .and(body_json(serde_json::json!([1, 2, 3])))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "count": 3,
        "items": [
            {"href": "/Apps/id/1", "id": 1, "name": "App 1"},
            {"href": "/Apps/id/2", "id": 2, "name": "App 2"},
            {"href": "/Apps/id/3", "id": 3, "name": "App 3"}
        ]
    })))
    .mount(&mock_server)
    .await;

  let result = client.batch_get_apps(&[1, 2, 3]).await.unwrap();
  assert_eq!(result.count, Some(3));
  assert_eq!(result.items.unwrap().len(), 3);
}
