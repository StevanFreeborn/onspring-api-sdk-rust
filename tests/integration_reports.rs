mod integration_common;

use integration_common::*;
use onspring::{DataFormat, PagingRequest, ReportDataType};

#[tokio::test]
#[ignore]
async fn get_report_should_return_report() {
  let client = build_client();
  let report_id = required_env_i32("TEST_REPORT");

  let report = client.get_report(report_id, None, None).await.unwrap();

  let columns = report.columns.unwrap();
  assert!(!columns.is_empty());

  let rows = report.rows.unwrap();
  assert!(!rows.is_empty());

  for row in &rows {
    assert!(row.record_id.unwrap() > 0);
    assert!(!row.cells.as_ref().unwrap().is_empty());
  }
}

#[tokio::test]
#[ignore]
async fn get_report_with_chart_data_should_return_data() {
  let client = build_client();
  let report_id = required_env_i32("TEST_REPORT_WITH_CHART_DATA");

  let report = client
    .get_report(
      report_id,
      Some(DataFormat::Raw),
      Some(ReportDataType::ChartData),
    )
    .await
    .unwrap();

  assert!(!report.columns.unwrap().is_empty());
  assert!(!report.rows.unwrap().is_empty());
}

#[tokio::test]
#[ignore]
async fn get_report_should_fail_for_chart_data_on_non_chart_report() {
  let client = build_client();
  let report_id = required_env_i32("TEST_REPORT");

  let result = client
    .get_report(
      report_id,
      Some(DataFormat::Raw),
      Some(ReportDataType::ChartData),
    )
    .await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_report_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.get_report(1, None, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn get_report_should_fail_when_not_found() {
  let client = build_client();
  let result = client.get_report(0, None, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn list_reports_should_return_reports() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");

  let response = client.list_reports(app_id, None).await.unwrap();

  assert!(response.total_records.unwrap() > 0);
  let items = response.items.unwrap();
  assert!(!items.is_empty());

  for report in &items {
    assert!(report.id > 0);
    assert!(report.app_id > 0);
    assert!(report.name.is_some());
  }
}

#[tokio::test]
#[ignore]
async fn list_reports_should_fail_with_invalid_api_key() {
  let client = build_client_with_key("invalid");
  let result = client.list_reports(1, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn list_reports_should_fail_when_no_access() {
  let client = build_client();
  let app_id = required_env_i32("TEST_APP_ID_NO_ACCESS");
  let result = client.list_reports(app_id, None).await;
  assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn list_reports_should_fail_with_invalid_page_size() {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let paging = PagingRequest {
    page_number: 1,
    page_size: 1001,
  };
  let result = client.list_reports(app_id, Some(paging)).await;
  assert!(result.is_err());
}
