mod integration_common;

use integration_common::*;
use onspring::PagingRequest;

#[tokio::test]
#[ignore]
async fn get_field_should_return_field() {
  let client = build_client();
  let field_id = required_env_i32("TEST_FIELD_ID");
  let field = client.get_field(field_id).await.unwrap();

  assert_eq!(field.id, field_id);
  assert!(field.name.is_some());
  assert!(field.app_id > 0);
}

#[tokio::test]
#[ignore]
async fn get_field_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.get_field(1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_field_should_fail_when_no_access() {
  let client = build_client();
  let field_id = required_env_i32("TEST_FIELD_ID_NO_ACCESS");
  let result = client.get_field(field_id).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_field_should_fail_when_not_found() {
  let client = build_client();
  let result = client.get_field(0).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn list_fields_should_return_fields() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let response = client.list_fields(app_id, None).await.unwrap();

  assert!(response.total_records.unwrap() > 0);
  let items = response.items.unwrap();
  assert!(!items.is_empty());

  for field in &items {
    assert!(field.id > 0);
    assert!(field.name.is_some());
    assert!(field.app_id > 0);
  }
}

#[tokio::test]
#[ignore]
async fn list_fields_should_respect_paging() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let paging = PagingRequest {
    page_number: 1,
    page_size: 1,
  };
  let response = client.list_fields(app_id, Some(paging)).await.unwrap();

  assert_eq!(response.page_number.unwrap(), 1);
  assert_eq!(response.page_size.unwrap(), 1);
  assert_eq!(response.items.unwrap().len(), 1);
}

#[tokio::test]
#[ignore]
async fn list_fields_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.list_fields(1, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn list_fields_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let result = client.list_fields(app_id, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_get_fields_should_return_fields() {
  let client = build_client();
  let field_ids = required_env_csv_i32("TEST_FIELD_IDS");
  let response = client.batch_get_fields(&field_ids).await.unwrap();

  let items = response.items.unwrap();
  assert_eq!(items.len(), field_ids.len());

  for field in &items {
    assert!(field.id > 0);
    assert!(field.name.is_some());
    assert!(field.app_id > 0);
  }
}

#[tokio::test]
#[ignore]
async fn batch_get_fields_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.batch_get_fields(&[1, 2, 3]).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_get_fields_should_fail_when_no_access() {
  let client = build_client();
  let field_ids = required_env_csv_i32("TEST_FIELD_IDS_NO_ACCESS");
  let result = client.batch_get_fields(&field_ids).await;
  assert!(result.is_err());
}
