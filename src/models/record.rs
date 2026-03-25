use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::enums::{DataFormat, ValueType};

/// Represents a record in an Onspring application.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub app_id: i32,
    pub record_id: i32,
    pub field_data: Option<Vec<RecordFieldValue>>,
}

/// Represents a single field value within a record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordFieldValue {
    #[serde(rename = "type")]
    pub value_type: ValueType,
    pub field_id: i32,
    pub value: serde_json::Value,
}

/// Request to create or update a record.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveRecordRequest {
    pub app_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<i32>,
    pub fields: HashMap<String, serde_json::Value>,
}

/// Response from saving a record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveRecordResponse {
    pub id: i32,
    pub warnings: Option<Vec<String>>,
}

/// Request to query records with a filter.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRecordsRequest {
    pub app_id: i32,
    pub filter: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<DataFormat>,
}

/// Request to get a batch of records.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchGetRecordsRequest {
    pub app_id: i32,
    pub record_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<DataFormat>,
}

/// Request to delete a batch of records.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteRecordsRequest {
    pub app_id: i32,
    pub record_ids: Vec<i32>,
}
