use serde::Deserialize;
use uuid::Uuid;

use super::enums::{FormulaOutputType, Multiplicity};

/// Represents a field in an Onspring application.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
  pub id: i32,
  pub app_id: i32,
  pub name: Option<String>,
  #[serde(rename = "type")]
  pub field_type: Option<String>,
  pub status: Option<String>,
  pub is_required: bool,
  pub is_unique: bool,
  pub multiplicity: Option<Multiplicity>,
  pub list_id: Option<i32>,
  pub values: Option<Vec<ListFieldValue>>,
  #[serde(rename = "outputType")]
  pub output_type: Option<FormulaOutputType>,
  pub referenced_app_id: Option<i32>,
}

/// Represents a value in a list field.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFieldValue {
  pub id: Uuid,
  pub name: String,
  pub sort_order: i32,
  pub numeric_value: Option<f64>,
  pub color: Option<String>,
}
