mod common;

use std::collections::HashMap;

use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_list_records() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Records/appId/1"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "pageNumber": 1,
        "pageSize": 50,
        "totalPages": 1,
        "totalRecords": 1,
        "items": [
            {
                "appId": 1,
                "recordId": 100,
                "fieldData": [
                    {"type": "String", "fieldId": 10, "value": "hello"}
                ]
            }
        ]
    })))
    .mount(&mock_server)
    .await;

  let result = client.list_records(1, None, None, None).await.unwrap();
  assert_eq!(result.total_records, Some(1));
  let items = result.items.unwrap();
  assert_eq!(items[0].record_id, 100);
  let field_data = items[0].field_data.as_ref().unwrap();
  assert_eq!(field_data[0].field_id, 10);
  assert_eq!(field_data[0].value, serde_json::json!("hello"));
}

#[tokio::test]
async fn test_list_records_with_params() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Records/appId/1"))
    .and(query_param("fieldIds", "10,20"))
    .and(query_param("dataFormat", "Formatted"))
    .and(query_param("PageNumber", "2"))
    .and(query_param("PageSize", "10"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "pageNumber": 2,
        "pageSize": 10,
        "totalPages": 5,
        "totalRecords": 50,
        "items": []
    })))
    .mount(&mock_server)
    .await;

  let paging = onspring::PagingRequest {
    page_number: 2,
    page_size: 10,
  };
  let result = client
    .list_records(
      1,
      Some(paging),
      Some(&[10, 20]),
      Some(onspring::DataFormat::Formatted),
    )
    .await
    .unwrap();
  assert_eq!(result.page_number, Some(2));
}

#[tokio::test]
async fn test_get_record() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Records/appId/1/recordId/42"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "appId": 1,
        "recordId": 42,
        "fieldData": [
            {"type": "Integer", "fieldId": 5, "value": 123}
        ]
    })))
    .mount(&mock_server)
    .await;

  let record = client.get_record(1, 42, None, None).await.unwrap();
  assert_eq!(record.record_id, 42);
  assert_eq!(record.app_id, 1);
}

#[tokio::test]
async fn test_save_record_create() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("PUT"))
    .and(path("/Records"))
    .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
        "id": 99,
        "warnings": []
    })))
    .mount(&mock_server)
    .await;

  let mut fields = HashMap::new();
  fields.insert("10".to_string(), serde_json::json!("test value"));

  let request = onspring::models::record::SaveRecordRequest {
    app_id: 1,
    record_id: None,
    fields,
  };
  let result = client.save_record(request).await.unwrap();
  assert_eq!(result.id, 99);
}

#[tokio::test]
async fn test_delete_record() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("DELETE"))
    .and(path("/Records/appId/1/recordId/42"))
    .respond_with(ResponseTemplate::new(204))
    .mount(&mock_server)
    .await;

  let result = client.delete_record(1, 42).await;
  assert!(result.is_ok());
}

#[tokio::test]
async fn test_batch_get_records() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("POST"))
    .and(path("/Records/batch-get"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "count": 2,
        "items": [
            {"appId": 1, "recordId": 1, "fieldData": []},
            {"appId": 1, "recordId": 2, "fieldData": []}
        ]
    })))
    .mount(&mock_server)
    .await;

  let request = onspring::models::record::BatchGetRecordsRequest {
    app_id: 1,
    record_ids: vec![1, 2],
    field_ids: None,
    data_format: None,
  };
  let result = client.batch_get_records(request).await.unwrap();
  assert_eq!(result.count, Some(2));
}

#[tokio::test]
async fn test_query_records() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("POST"))
    .and(path("/Records/Query"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "pageNumber": 1,
        "pageSize": 50,
        "totalPages": 1,
        "totalRecords": 1,
        "items": [
            {"appId": 1, "recordId": 5, "fieldData": []}
        ]
    })))
    .mount(&mock_server)
    .await;

  let request = onspring::models::record::QueryRecordsRequest {
    app_id: 1,
    filter: "fieldId eq 'test'".to_string(),
    field_ids: None,
    data_format: None,
  };
  let result = client.query_records(request, None).await.unwrap();
  assert_eq!(result.total_records, Some(1));
}

#[tokio::test]
async fn test_batch_delete_records() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("POST"))
    .and(path("/Records/batch-delete"))
    .respond_with(ResponseTemplate::new(204))
    .mount(&mock_server)
    .await;

  let request = onspring::models::record::BatchDeleteRecordsRequest {
    app_id: 1,
    record_ids: vec![1, 2, 3],
  };
  let result = client.batch_delete_records(request).await;
  assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_record_not_found() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Records/appId/1/recordId/999"))
    .respond_with(ResponseTemplate::new(404))
    .mount(&mock_server)
    .await;

  let result = client.get_record(1, 999, None, None).await;
  assert!(result.is_err());
  if let Err(onspring::OnspringError::Api { status_code, .. }) = result {
    assert_eq!(status_code, 404);
  } else {
    panic!("Expected Api error");
  }
}
