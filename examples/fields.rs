use onspring::{OnspringClient, PagingRequest};

#[tokio::main]
async fn main() -> onspring::Result<()> {
  dotenvy::dotenv().ok();
  let base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL is required");
  let api_key = std::env::var("SANDBOX_API_KEY").expect("SANDBOX_API_KEY is required");
  let app_id: i32 = std::env::var("TEST_APP_ID")
    .expect("TEST_APP_ID is required")
    .parse()
    .expect("TEST_APP_ID must be an integer");

  let client = OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build();

  // List fields for an app
  let paging = PagingRequest {
    page_number: 1,
    page_size: 10,
  };
  let response = client.list_fields(app_id, Some(paging)).await?;
  println!("Total fields: {:?}", response.total_records);

  if let Some(items) = &response.items {
    for field in items {
      println!(
        "  Field: {} (id: {}, type: {:?})",
        field.name.as_deref().unwrap_or("N/A"),
        field.id,
        field.field_type.as_deref().unwrap_or("N/A"),
      );
    }

    // Get a single field
    if let Some(first) = items.first() {
      let field = client.get_field(first.id).await?;
      println!(
        "\nFetched field: {} (id: {})",
        field.name.as_deref().unwrap_or("N/A"),
        field.id
      );
    }

    // Batch get fields
    let ids: Vec<i32> = items.iter().take(3).map(|f| f.id).collect();
    if !ids.is_empty() {
      let batch = client.batch_get_fields(&ids).await?;
      println!(
        "\nBatch fetched {} fields",
        batch.items.map(|i| i.len()).unwrap_or(0)
      );
    }
  }

  Ok(())
}
