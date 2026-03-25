mod common;

use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_get_report() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Reports/id/1"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "columns": ["Record Id", "Name", "Status"],
        "rows": [
            {"recordId": 100, "cells": [100, "Test", "Active"]},
            {"recordId": 101, "cells": [101, "Other", "Inactive"]}
        ]
    })))
    .mount(&mock_server)
    .await;

  let report = client.get_report(1, None, None).await.unwrap();
  let columns = report.columns.unwrap();
  assert_eq!(columns.len(), 3);
  assert_eq!(columns[0], "Record Id");
  let rows = report.rows.unwrap();
  assert_eq!(rows.len(), 2);
  assert_eq!(rows[0].record_id, Some(100));
}

#[tokio::test]
async fn test_get_report_with_format() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Reports/id/1"))
    .and(query_param("apiDataFormat", "Formatted"))
    .and(query_param("dataType", "ChartData"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "columns": ["Name"],
        "rows": []
    })))
    .mount(&mock_server)
    .await;

  let result = client
    .get_report(
      1,
      Some(onspring::DataFormat::Formatted),
      Some(onspring::ReportDataType::ChartData),
    )
    .await
    .unwrap();
  assert!(result.columns.unwrap().len() == 1);
}

#[tokio::test]
async fn test_list_reports() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Reports/appId/5"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "pageNumber": 1,
        "pageSize": 50,
        "totalPages": 1,
        "totalRecords": 2,
        "items": [
            {"appId": 5, "id": 10, "name": "Report A", "description": "First report"},
            {"appId": 5, "id": 11, "name": "Report B", "description": null}
        ]
    })))
    .mount(&mock_server)
    .await;

  let result = client.list_reports(5, None).await.unwrap();
  assert_eq!(result.total_records, Some(2));
  let items = result.items.unwrap();
  assert_eq!(items[0].id, 10);
  assert_eq!(items[0].name.as_deref(), Some("Report A"));
  assert_eq!(items[1].description, None);
}

#[tokio::test]
async fn test_get_report_not_found() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Reports/id/999"))
    .respond_with(ResponseTemplate::new(404))
    .mount(&mock_server)
    .await;

  let result = client.get_report(999, None, None).await;
  assert!(result.is_err());
  if let Err(onspring::OnspringError::Api { status_code, .. }) = result {
    assert_eq!(status_code, 404);
  } else {
    panic!("Expected Api error");
  }
}
