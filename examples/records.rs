use onspring::{OnspringClient, PagingRequest, QueryRecordsRequest, SaveRecordRequest};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> onspring::Result<()> {
  dotenvy::dotenv().ok();
  let base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL is required");
  let api_key = std::env::var("SANDBOX_API_KEY").expect("SANDBOX_API_KEY is required");
  let app_id: i32 = std::env::var("TEST_SURVEY_ID")
    .expect("TEST_SURVEY_ID is required")
    .parse()
    .expect("TEST_SURVEY_ID must be an integer");
  let field_id = std::env::var("TEST_TEXT_FIELD").expect("TEST_TEXT_FIELD is required");

  let client = OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build();

  // List records
  let paging = PagingRequest {
    page_number: 1,
    page_size: 5,
  };
  let response = client
    .list_records(app_id, Some(paging), None, None)
    .await?;
  println!("Total records: {:?}", response.total_records);

  if let Some(items) = &response.items {
    for record in items {
      println!("  Record ID: {}", record.record_id);
    }
  }

  // Create a record
  let mut fields = HashMap::new();
  fields.insert(field_id.clone(), serde_json::json!("Hello from Rust SDK"));

  let request = SaveRecordRequest {
    app_id,
    record_id: None,
    fields,
  };

  let saved = client.save_record(request).await?;
  println!("\nCreated record with ID: {}", saved.id);

  // Get the record back
  let record = client.get_record(app_id, saved.id, None, None).await?;
  println!(
    "Fetched record: app_id={}, record_id={}",
    record.app_id, record.record_id
  );

  // Update the record
  let mut fields = HashMap::new();
  fields.insert(field_id, serde_json::json!("Updated from Rust SDK"));

  let update = SaveRecordRequest {
    app_id,
    record_id: Some(saved.id),
    fields,
  };
  client.save_record(update).await?;
  println!("Updated record {}", saved.id);

  // Query records
  let query = QueryRecordsRequest {
    app_id,
    filter: format!(
      "{} contains 'Rust'",
      std::env::var("TEST_TEXT_FIELD").unwrap()
    ),
    field_ids: None,
    data_format: None,
  };
  let query_result = client.query_records(query, None).await?;
  println!("\nQuery found {:?} records", query_result.total_records);

  // Delete the record
  client.delete_record(app_id, saved.id).await?;
  println!("Deleted record {}", saved.id);

  Ok(())
}
