use serde::Deserialize;

/// Represents an Onspring application.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
  pub href: Option<String>,
  pub id: i32,
  pub name: Option<String>,
}
