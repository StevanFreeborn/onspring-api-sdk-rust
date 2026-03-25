mod integration_common;

use integration_common::*;
use onspring::{
  BatchDeleteRecordsRequest, BatchGetRecordsRequest, DataFormat, PagingRequest,
  QueryRecordsRequest, SaveRecordRequest,
};
use std::collections::HashMap;

#[tokio::test]
#[ignore]
async fn list_records_should_return_records() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let response = client.list_records(app_id, None, None, None).await.unwrap();

  assert!(response.total_records.unwrap() > 0);
  let items = response.items.unwrap();
  assert!(!items.is_empty());

  for record in &items {
    assert_eq!(record.app_id, app_id);
    assert!(record.record_id > 0);
    assert!(record.field_data.is_some());
  }
}

#[tokio::test]
#[ignore]
async fn list_records_with_params_should_return_records() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let field_id = required_env_i32("TEST_TEXT_FIELD");
  let paging = PagingRequest {
    page_number: 1,
    page_size: 1,
  };

  let response = client
    .list_records(
      app_id,
      Some(paging),
      Some(&[field_id]),
      Some(DataFormat::Formatted),
    )
    .await
    .unwrap();

  assert_eq!(response.page_number.unwrap(), 1);
  assert_eq!(response.page_size.unwrap(), 1);

  for record in response.items.unwrap_or_default() {
    assert_eq!(record.app_id, app_id);
    assert!(record.field_data.is_some());
  }
}

#[tokio::test]
#[ignore]
async fn list_records_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.list_records(1, None, None, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn list_records_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let result = client.list_records(app_id, None, None, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_record_should_return_record() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let record_id = required_env_i32("TEST_SURVEY_RECORD_ID");

  let record = client
    .get_record(app_id, record_id, None, None)
    .await
    .unwrap();

  assert_eq!(record.app_id, app_id);
  assert_eq!(record.record_id, record_id);
  assert!(record.field_data.is_some());
}

#[tokio::test]
#[ignore]
async fn get_record_with_params_should_return_record() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let record_id = required_env_i32("TEST_SURVEY_RECORD_ID");
  let field_id = required_env_i32("TEST_TEXT_FIELD");

  let record = client
    .get_record(
      app_id,
      record_id,
      Some(&[field_id]),
      Some(DataFormat::Formatted),
    )
    .await
    .unwrap();

  assert_eq!(record.app_id, app_id);
  assert_eq!(record.record_id, record_id);
  assert!(record.field_data.is_some());
}

#[tokio::test]
#[ignore]
async fn get_record_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.get_record(1, 1, None, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_record_should_fail_when_not_found() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let result = client.get_record(app_id, 0, None, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn save_record_should_create_record() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let field_id = required_env("TEST_TEXT_FIELD");

  let mut fields = HashMap::new();
  fields.insert(field_id, serde_json::json!("integration test create"));

  let request = SaveRecordRequest {
    app_id,
    record_id: None,
    fields,
  };

  let response = client.save_record(request).await.unwrap();
  assert!(response.id > 0);

  // Cleanup
  let _ = client.delete_record(app_id, response.id).await;
}

#[tokio::test]
#[ignore]
async fn save_record_should_update_record() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let field_id = required_env("TEST_TEXT_FIELD");

  // Create a record first
  let mut fields = HashMap::new();
  fields.insert(field_id.clone(), serde_json::json!("integration test"));
  let create_request = SaveRecordRequest {
    app_id,
    record_id: None,
    fields,
  };
  let created = client.save_record(create_request).await.unwrap();

  // Update it
  let mut fields = HashMap::new();
  fields.insert(field_id, serde_json::json!("updated"));
  let update_request = SaveRecordRequest {
    app_id,
    record_id: Some(created.id),
    fields,
  };
  let response = client.save_record(update_request).await.unwrap();
  assert_eq!(response.id, created.id);

  // Cleanup
  let _ = client.delete_record(app_id, created.id).await;
}

#[tokio::test]
#[ignore]
async fn save_record_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let request = SaveRecordRequest {
    app_id: 0,
    record_id: None,
    fields: HashMap::new(),
  };
  let result = client.save_record(request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn save_record_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let request = SaveRecordRequest {
    app_id,
    record_id: None,
    fields: HashMap::new(),
  };
  let result = client.save_record(request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_record_should_succeed() {
  let record_id = add_record().await;
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");

  client.delete_record(app_id, record_id).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_record_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.delete_record(1, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_record_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let result = client.delete_record(app_id, 1).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn delete_record_should_fail_when_not_found() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID");
  let result = client.delete_record(app_id, 0).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_get_records_should_return_records() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let record_id = required_env_i32("TEST_SURVEY_RECORD_ID");

  let request = BatchGetRecordsRequest {
    app_id,
    record_ids: vec![record_id],
    field_ids: None,
    data_format: None,
  };

  let response = client.batch_get_records(request).await.unwrap();
  let items = response.items.unwrap();
  assert!(!items.is_empty());

  for record in &items {
    assert_eq!(record.app_id, app_id);
    assert!(record.field_data.is_some());
  }
}

#[tokio::test]
#[ignore]
async fn batch_get_records_with_params_should_return_records() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let record_id = required_env_i32("TEST_SURVEY_RECORD_ID");
  let field_id = required_env_i32("TEST_TEXT_FIELD");

  let request = BatchGetRecordsRequest {
    app_id,
    record_ids: vec![record_id],
    field_ids: Some(vec![field_id]),
    data_format: Some(DataFormat::Formatted),
  };

  let response = client.batch_get_records(request).await.unwrap();
  let items = response.items.unwrap();
  assert!(!items.is_empty());
}

#[tokio::test]
#[ignore]
async fn batch_get_records_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let request = BatchGetRecordsRequest {
    app_id: 1,
    record_ids: vec![1],
    field_ids: None,
    data_format: None,
  };
  let result = client.batch_get_records(request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_get_records_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let request = BatchGetRecordsRequest {
    app_id,
    record_ids: vec![1],
    field_ids: None,
    data_format: None,
  };
  let result = client.batch_get_records(request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn query_records_should_return_records() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let field_id = required_env_i32("TEST_SURVEY_AUTO_NUMBER_FIELD");

  let filter = format!("{} gt 0", field_id);
  let request = QueryRecordsRequest {
    app_id,
    filter,
    field_ids: None,
    data_format: None,
  };

  let response = client.query_records(request, None).await.unwrap();
  assert!(response.total_records.unwrap() > 0);

  for record in response.items.unwrap_or_default() {
    assert_eq!(record.app_id, app_id);
  }
}

#[tokio::test]
#[ignore]
async fn query_records_with_params_should_return_records() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let field_id = required_env_i32("TEST_SURVEY_AUTO_NUMBER_FIELD");

  let filter = format!("{} gt 0", field_id);
  let paging = PagingRequest {
    page_number: 1,
    page_size: 1,
  };
  let request = QueryRecordsRequest {
    app_id,
    filter,
    field_ids: Some(vec![field_id]),
    data_format: Some(DataFormat::Formatted),
  };

  let response = client.query_records(request, Some(paging)).await.unwrap();
  assert_eq!(response.page_size.unwrap(), 1);
}

#[tokio::test]
#[ignore]
async fn query_records_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let request = QueryRecordsRequest {
    app_id: 1,
    filter: String::new(),
    field_ids: None,
    data_format: None,
  };
  let result = client.query_records(request, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn query_records_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let field_id = required_env_i32("TEST_SURVEY_AUTO_NUMBER_FIELD");

  let filter = format!("{} gt 0", field_id);
  let request = QueryRecordsRequest {
    app_id,
    filter,
    field_ids: None,
    data_format: None,
  };
  let result = client.query_records(request, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_delete_records_should_succeed() {
  let record_id1 = add_record().await;
  let record_id2 = add_record().await;

  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");

  let request = BatchDeleteRecordsRequest {
    app_id,
    record_ids: vec![record_id1, record_id2],
  };

  client.batch_delete_records(request).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn batch_delete_records_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let request = BatchDeleteRecordsRequest {
    app_id: 1,
    record_ids: vec![1],
  };
  let result = client.batch_delete_records(request).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn batch_delete_records_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let request = BatchDeleteRecordsRequest {
    app_id,
    record_ids: vec![1],
  };
  let result = client.batch_delete_records(request).await;
  assert!(result.is_err());
}
