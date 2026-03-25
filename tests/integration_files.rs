mod integration_common;

use integration_common::*;
use onspring::SaveFileRequest;

#[tokio::test]
#[ignore]
async fn get_file_info_for_attachment_should_return_info() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");
  let file_id = required_env_i32("TEST_ATTACHMENT");

  let info = client
    .get_file_info(record_id, field_id, file_id)
    .await
    .unwrap();

  assert!(info.name.is_some());
  assert!(info.content_type.is_some());
  assert!(info.created_date.is_some());
  assert!(info.modified_date.is_some());
  assert!(info.file_href.is_some());
  assert!(info.owner.is_some());
  assert!(info.file_type.is_some());
}

#[tokio::test]
#[ignore]
async fn get_file_info_for_image_should_return_info() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_IMAGE_FIELD");
  let file_id = required_env_i32("TEST_IMAGE");

  let info = client
    .get_file_info(record_id, field_id, file_id)
    .await
    .unwrap();

  assert!(info.name.is_some());
  assert!(info.content_type.is_some());
  assert!(info.created_date.is_some());
  assert!(info.modified_date.is_some());
  assert!(info.file_href.is_some());
  assert!(info.owner.is_some());
  assert!(info.file_type.is_some());
}

#[tokio::test]
#[ignore]
async fn get_file_info_should_fail_for_non_file_field() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_TEXT_FIELD");
  let file_id = required_env_i32("TEST_ATTACHMENT");

  let result = client.get_file_info(record_id, field_id, file_id).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_file_info_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");
  let file_id = required_env_i32("TEST_ATTACHMENT");

  let result = client.get_file_info(record_id, field_id, file_id).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_file_info_should_fail_when_no_field_access() {
  let client = build_client();
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD_NO_ACCESS_FIELD");
  let result = client.get_file_info(1, field_id, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_file_info_should_fail_when_no_app_access() {
  let client = build_client();
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD_NO_ACCESS_APP");
  let result = client.get_file_info(1, field_id, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_file_info_should_fail_when_field_not_found() {
  let client = build_client();
  let result = client.get_file_info(1, 0, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_file_info_should_fail_when_record_not_found() {
  let client = build_client();
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");
  let result = client.get_file_info(0, field_id, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_file_for_attachment_should_return_file() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");
  let file_id = required_env_i32("TEST_ATTACHMENT");

  let response = client.get_file(record_id, field_id, file_id).await.unwrap();

  assert!(response.content_type.is_some());
  assert!(response.file_name.is_some());
  assert!(!response.data.is_empty());
}

#[tokio::test]
#[ignore]
async fn get_file_for_image_should_return_file() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_IMAGE_FIELD");
  let file_id = required_env_i32("TEST_IMAGE");

  let response = client.get_file(record_id, field_id, file_id).await.unwrap();

  assert!(response.content_type.is_some());
  assert!(response.file_name.is_some());
  assert!(!response.data.is_empty());
}

#[tokio::test]
#[ignore]
async fn get_file_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");
  let file_id = required_env_i32("TEST_ATTACHMENT");

  let result = client.get_file(record_id, field_id, file_id).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn upload_file_to_attachment_field_should_succeed() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");

  let file_data = std::fs::read("tests/test_data/test-attachment.txt").unwrap();

  let request = SaveFileRequest {
    record_id,
    field_id,
    notes: Some("integration test".to_string()),
    modified_date: Some(chrono::Utc::now()),
    file_name: "test-attachment.txt".to_string(),
    file_data,
    content_type: "text/plain".to_string(),
  };

  let response = client.upload_file(request).await.unwrap();
  assert!(response.id > 0);

  // Cleanup
  let _ = client.delete_file(record_id, field_id, response.id).await;
}

#[tokio::test]
#[ignore]
async fn upload_file_to_image_field_should_succeed() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_IMAGE_FIELD");

  let file_data = std::fs::read("tests/test_data/test-image.jpeg").unwrap();

  let request = SaveFileRequest {
    record_id,
    field_id,
    notes: Some("integration test".to_string()),
    modified_date: Some(chrono::Utc::now()),
    file_name: "test-image.jpeg".to_string(),
    file_data,
    content_type: "image/jpeg".to_string(),
  };

  let response = client.upload_file(request).await.unwrap();
  assert!(response.id > 0);

  // Cleanup
  let _ = client.delete_file(record_id, field_id, response.id).await;
}

#[tokio::test]
#[ignore]
async fn upload_file_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");

  let request = SaveFileRequest {
    record_id: 1,
    field_id: 1,
    notes: Some("test".to_string()),
    modified_date: Some(chrono::Utc::now()),
    file_name: "test.txt".to_string(),
    file_data: b"test".to_vec(),
    content_type: "text/plain".to_string(),
  };

  let result = client.upload_file(request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn upload_file_should_fail_for_non_file_field() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_TEXT_FIELD");

  let file_data = std::fs::read("tests/test_data/test-attachment.txt").unwrap();

  let request = SaveFileRequest {
    record_id,
    field_id,
    notes: Some("test".to_string()),
    modified_date: Some(chrono::Utc::now()),
    file_name: "test-attachment.txt".to_string(),
    file_data,
    content_type: "text/plain".to_string(),
  };

  let result = client.upload_file(request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_file_from_attachment_should_succeed() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");

  // Upload first
  let file_data = std::fs::read("tests/test_data/test-attachment.txt").unwrap();
  let request = SaveFileRequest {
    record_id,
    field_id,
    notes: Some("to be deleted".to_string()),
    modified_date: Some(chrono::Utc::now()),
    file_name: "test-attachment.txt".to_string(),
    file_data,
    content_type: "text/plain".to_string(),
  };
  let uploaded = client.upload_file(request).await.unwrap();

  // Delete
  client
    .delete_file(record_id, field_id, uploaded.id)
    .await
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_file_from_image_should_succeed() {
  let client = build_client();
  let record_id = required_env_i32("TEST_RECORD");
  let field_id = required_env_i32("TEST_IMAGE_FIELD");

  // Upload first
  let file_data = std::fs::read("tests/test_data/test-image.jpeg").unwrap();
  let request = SaveFileRequest {
    record_id,
    field_id,
    notes: Some("to be deleted".to_string()),
    modified_date: Some(chrono::Utc::now()),
    file_name: "test-image.jpeg".to_string(),
    file_data,
    content_type: "image/jpeg".to_string(),
  };
  let uploaded = client.upload_file(request).await.unwrap();

  // Delete
  client
    .delete_file(record_id, field_id, uploaded.id)
    .await
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_file_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.delete_file(1, 1, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_file_should_fail_when_no_field_access() {
  let client = build_client();
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD_NO_ACCESS_FIELD");
  let result = client.delete_file(1, field_id, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_file_should_fail_when_no_app_access() {
  let client = build_client();
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD_NO_ACCESS_APP");
  let result = client.delete_file(1, field_id, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_file_should_fail_when_field_not_found() {
  let client = build_client();
  let result = client.delete_file(1, 0, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_file_should_fail_when_record_not_found() {
  let client = build_client();
  let field_id = required_env_i32("TEST_ATTACHMENT_FIELD");
  let result = client.delete_file(0, field_id, 1).await;
  assert!(result.is_err());
}
