use onspring::OnspringClient;
use std::collections::HashMap;

pub fn required_env(name: &str) -> String {
  std::env::var(name).unwrap_or_else(|_| panic!("{} is not defined", name))
}

pub fn required_env_i32(name: &str) -> i32 {
  required_env(name)
    .parse()
    .unwrap_or_else(|_| panic!("{} is not a valid integer", name))
}

pub fn required_env_csv_i32(name: &str) -> Vec<i32> {
  required_env(name)
    .split(',')
    .map(|s| {
      s.trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} contains non-integer values", name))
    })
    .collect()
}

pub fn build_client() -> OnspringClient {
  let base_url = required_env("API_BASE_URL");
  let api_key = required_env("SANDBOX_API_KEY");
  OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build()
}

pub fn build_client_with_key(api_key: &str) -> OnspringClient {
  let base_url = required_env("API_BASE_URL");
  OnspringClient::builder(api_key).base_url(&base_url).build()
}

pub async fn add_record() -> i32 {
  let client = build_client();
  let app_id = required_env_i32("TEST_SURVEY_ID");
  let field_id = required_env("TEST_TEXT_FIELD");

  let mut fields = HashMap::new();
  fields.insert(field_id, serde_json::json!("test"));

  let request = onspring::SaveRecordRequest {
    app_id,
    record_id: None,
    fields,
  };

  let response = client.save_record(request).await.unwrap();
  response.id
}
