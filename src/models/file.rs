use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Information about a file attachment.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
  #[serde(rename = "type")]
  pub file_type: Option<String>,
  pub content_type: Option<String>,
  pub name: Option<String>,
  pub created_date: Option<DateTime<Utc>>,
  pub modified_date: Option<DateTime<Utc>>,
  pub owner: Option<String>,
  pub notes: Option<String>,
  pub file_href: Option<String>,
}

/// The content of a downloaded file.
#[derive(Debug, Clone)]
pub struct FileResponse {
  pub content_type: Option<String>,
  pub file_name: Option<String>,
  pub data: Bytes,
}

/// Request to upload a file.
#[derive(Debug, Clone)]
pub struct SaveFileRequest {
  pub record_id: i32,
  pub field_id: i32,
  pub notes: Option<String>,
  pub modified_date: Option<DateTime<Utc>>,
  pub file_name: String,
  pub file_data: Vec<u8>,
  pub content_type: String,
}

/// Response from creating a file, containing the new file ID.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedWithIdResponse {
  pub id: i32,
}
