mod integration_common;

use integration_common::*;
use onspring::PagingRequest;

#[tokio::test]
#[ignore]
async fn list_apps_should_return_apps() {
  let client = build_client();
  let response = client.list_apps(None).await.unwrap();

  assert!(response.total_records.unwrap() > 0);
  let items = response.items.unwrap();
  assert!(!items.is_empty());

  for app in &items {
    assert!(app.id > 0);
    assert!(app.name.is_some());
    assert!(app.href.is_some());
  }
}

#[tokio::test]
#[ignore]
async fn list_apps_should_respect_paging() {
  let client = build_client();
  let paging = PagingRequest {
    page_number: 1,
    page_size: 1,
  };
  let response = client.list_apps(Some(paging)).await.unwrap();

  assert_eq!(response.page_number.unwrap(), 1);
  assert_eq!(response.page_size.unwrap(), 1);
  assert_eq!(response.items.unwrap().len(), 1);
}

#[tokio::test]
#[ignore]
async fn list_apps_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.list_apps(None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_app_should_return_app() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID");
  let app = client.get_app(app_id).await.unwrap();

  assert_eq!(app.id, app_id);
  assert!(app.name.is_some());
  assert!(app.href.is_some());
}

#[tokio::test]
#[ignore]
async fn get_app_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.get_app(1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_app_should_fail_when_not_found() {
  let client = build_client();
  let result = client.get_app(0).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_app_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let result = client.get_app(app_id).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_get_apps_should_return_apps() {
  let client = build_client();
  let app_ids = required_env_csv_i32("TEST_APP_IDS");
  let response = client.batch_get_apps(&app_ids).await.unwrap();

  let items = response.items.unwrap();
  assert_eq!(items.len(), app_ids.len());

  for app in &items {
    assert!(app.id > 0);
    assert!(app.name.is_some());
    assert!(app.href.is_some());
  }
}

#[tokio::test]
#[ignore]
async fn batch_get_apps_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.batch_get_apps(&[1]).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_get_apps_should_fail_when_no_access() {
  let client = build_client();
  let app_ids = required_env_csv_i32("TEST_APP_IDS_NO_ACCESS");
  let result = client.batch_get_apps(&app_ids).await;
  assert!(result.is_err());
}
