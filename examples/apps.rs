use onspring::{OnspringClient, PagingRequest};

#[tokio::main]
async fn main() -> onspring::Result<()> {
  let base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL is required");
  let api_key = std::env::var("SANDBOX_API_KEY").expect("SANDBOX_API_KEY is required");

  let client = OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build();

  // List apps with paging
  let paging = PagingRequest {
    page_number: 1,
    page_size: 10,
  };
  let response = client.list_apps(Some(paging)).await?;
  println!("Total apps: {:?}", response.total_records);

  if let Some(items) = &response.items {
    for app in items {
      println!(
        "  App: {} (id: {})",
        app.name.as_deref().unwrap_or("N/A"),
        app.id
      );
    }

    // Get a single app by ID
    if let Some(first) = items.first() {
      let app = client.get_app(first.id).await?;
      println!(
        "\nFetched app: {} (id: {})",
        app.name.as_deref().unwrap_or("N/A"),
        app.id
      );
    }

    // Batch get apps
    let ids: Vec<i32> = items.iter().take(3).map(|a| a.id).collect();
    if !ids.is_empty() {
      let batch = client.batch_get_apps(&ids).await?;
      println!(
        "\nBatch fetched {} apps",
        batch.items.map(|i| i.len()).unwrap_or(0)
      );
    }
  }

  Ok(())
}
