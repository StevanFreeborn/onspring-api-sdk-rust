use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to create or update a list item.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveListItemRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Uuid>,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub numeric_value: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub color: Option<String>,
}

/// Response from saving a list item.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveListItemResponse {
  pub id: Uuid,
}
