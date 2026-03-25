use serde::Deserialize;

/// Represents a report associated to an app (used in list responses).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportInfo {
  pub app_id: i32,
  pub id: i32,
  pub name: Option<String>,
  pub description: Option<String>,
}

/// Represents the data returned by a report.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportData {
  pub columns: Option<Vec<String>>,
  pub rows: Option<Vec<ReportRow>>,
}

/// Represents a single row in a report.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportRow {
  pub record_id: Option<i32>,
  pub cells: Option<Vec<serde_json::Value>>,
}
