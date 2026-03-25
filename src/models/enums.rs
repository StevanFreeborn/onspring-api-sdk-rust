use serde::{Deserialize, Serialize};

/// The format of data returned by the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataFormat {
  Raw,
  Formatted,
}

/// The type of report data to retrieve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportDataType {
  ReportData,
  ChartData,
}

/// The type of a field's value in a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
  String,
  Integer,
  Decimal,
  Date,
  TimeSpan,
  Guid,
  StringList,
  IntegerList,
  GuidList,
  AttachmentList,
  ScoringGroupList,
  FileList,
}

/// The output type of a formula field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormulaOutputType {
  Text,
  Numeric,
  DateAndTime,
  ListValue,
}

/// Whether a field allows single or multiple selections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Multiplicity {
  SingleSelect,
  MultiSelect,
}
