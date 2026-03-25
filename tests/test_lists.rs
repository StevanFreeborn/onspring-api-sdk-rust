mod common;

use uuid::Uuid;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_save_list_item_create() {
  let (mock_server, client) = common::setup().await;
  let new_id = Uuid::new_v4();

  Mock::given(method("PUT"))
    .and(path("/Lists/id/10/items"))
    .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
        "id": new_id.to_string()
    })))
    .mount(&mock_server)
    .await;

  let request = onspring::models::list::SaveListItemRequest {
    id: None,
    name: "New Item".to_string(),
    numeric_value: Some(1.0),
    color: Some("#ff0000".to_string()),
  };
  let result = client.save_list_item(10, request).await.unwrap();
  assert_eq!(result.id, new_id);
}

#[tokio::test]
async fn test_save_list_item_update() {
  let (mock_server, client) = common::setup().await;
  let existing_id = Uuid::new_v4();

  Mock::given(method("PUT"))
    .and(path("/Lists/id/10/items"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": existing_id.to_string()
    })))
    .mount(&mock_server)
    .await;

  let request = onspring::models::list::SaveListItemRequest {
    id: Some(existing_id),
    name: "Updated Item".to_string(),
    numeric_value: None,
    color: None,
  };
  let result = client.save_list_item(10, request).await.unwrap();
  assert_eq!(result.id, existing_id);
}

#[tokio::test]
async fn test_delete_list_item() {
  let (mock_server, client) = common::setup().await;
  let item_id = Uuid::new_v4();

  Mock::given(method("DELETE"))
    .and(path(format!("/Lists/id/10/itemId/{}", item_id)))
    .respond_with(ResponseTemplate::new(204))
    .mount(&mock_server)
    .await;

  let result = client.delete_list_item(10, item_id).await;
  assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_list_item_not_found() {
  let (mock_server, client) = common::setup().await;
  let item_id = Uuid::new_v4();

  Mock::given(method("DELETE"))
    .and(path(format!("/Lists/id/10/itemId/{}", item_id)))
    .respond_with(ResponseTemplate::new(404))
    .mount(&mock_server)
    .await;

  let result = client.delete_list_item(10, item_id).await;
  assert!(result.is_err());
  if let Err(onspring::OnspringError::Api { status_code, .. }) = result {
    assert_eq!(status_code, 404);
  } else {
    panic!("Expected Api error");
  }
}
