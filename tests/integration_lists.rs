mod integration_common;

use integration_common::*;
use onspring::SaveListItemRequest;

#[tokio::test]
#[ignore]
async fn save_list_item_should_add_item() {
  let client = build_client();
  let list_id = required_env_i32("TEST_LIST_ID");

  let name = format!("added_list_value_{}", chrono::Utc::now().timestamp_millis());
  let request = SaveListItemRequest {
    id: None,
    name,
    numeric_value: Some(1.0),
    color: Some("#000000".to_string()),
  };

  let response = client.save_list_item(list_id, request).await.unwrap();
  let item_id = response.id;

  // Cleanup
  let _ = client.delete_list_item(list_id, item_id).await;
}

#[tokio::test]
#[ignore]
async fn save_list_item_should_update_item() {
  let client = build_client();
  let list_id = required_env_i32("TEST_LIST_ID");

  // Create
  let name = format!("added_list_value_{}", chrono::Utc::now().timestamp_millis());
  let request = SaveListItemRequest {
    id: None,
    name,
    numeric_value: None,
    color: None,
  };
  let created = client.save_list_item(list_id, request).await.unwrap();

  // Update
  let name = format!(
    "updated_list_value_{}",
    chrono::Utc::now().timestamp_millis()
  );
  let update_request = SaveListItemRequest {
    id: Some(created.id),
    name,
    numeric_value: Some(1.0),
    color: Some("#000000".to_string()),
  };
  let _response = client
    .save_list_item(list_id, update_request)
    .await
    .unwrap();

  // Cleanup
  let _ = client.delete_list_item(list_id, created.id).await;
}

#[tokio::test]
#[ignore]
async fn save_list_item_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let request = SaveListItemRequest {
    id: None,
    name: "test".to_string(),
    numeric_value: Some(1.0),
    color: Some("#000000".to_string()),
  };
  let result = client.save_list_item(1, request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn save_list_item_should_fail_when_list_not_found() {
  let client = build_client();
  let request = SaveListItemRequest {
    id: None,
    name: "test".to_string(),
    numeric_value: Some(1.0),
    color: Some("#000000".to_string()),
  };
  let result = client.save_list_item(0, request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn save_list_item_should_fail_when_item_not_found() {
  let client = build_client();
  let list_id = required_env_i32("TEST_LIST_ID");
  let fake_uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap();

  let request = SaveListItemRequest {
    id: Some(fake_uuid),
    name: "test".to_string(),
    numeric_value: Some(1.0),
    color: Some("#000000".to_string()),
  };
  let result = client.save_list_item(list_id, request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_list_item_should_succeed() {
  let client = build_client();
  let list_id = required_env_i32("TEST_LIST_ID");

  // Create first
  let name = format!("to_delete_{}", chrono::Utc::now().timestamp_millis());
  let request = SaveListItemRequest {
    id: None,
    name,
    numeric_value: Some(1.0),
    color: Some("#000000".to_string()),
  };
  let created = client.save_list_item(list_id, request).await.unwrap();

  // Delete
  client.delete_list_item(list_id, created.id).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_list_item_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let fake_uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap();
  let result = client.delete_list_item(1, fake_uuid).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_list_item_should_fail_when_no_access() {
  let client = build_client();
  let list_id = required_env_i32("TEST_LIST_ID_NO_ACCESS");
  let item_id = required_env("TEST_LIST_ITEM_ID_NO_ACCESS");
  let uuid = uuid::Uuid::parse_str(&item_id).unwrap();

  let result = client.delete_list_item(list_id, uuid).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_list_item_should_fail_when_list_not_found() {
  let client = build_client();
  let fake_uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap();
  let result = client.delete_list_item(0, fake_uuid).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_list_item_should_fail_when_item_not_found() {
  let client = build_client();
  let list_id = required_env_i32("TEST_LIST_ID");
  let fake_uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap();
  let result = client.delete_list_item(list_id, fake_uuid).await;
  assert!(result.is_err());
}
