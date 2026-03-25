use onspring::{OnspringClient, SaveListItemRequest};

#[tokio::main]
async fn main() -> onspring::Result<()> {
  let base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL is required");
  let api_key = std::env::var("SANDBOX_API_KEY").expect("SANDBOX_API_KEY is required");
  let list_id: i32 = std::env::var("TEST_LIST_ID")
    .expect("TEST_LIST_ID is required")
    .parse()
    .expect("TEST_LIST_ID must be an integer");

  let client = OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build();

  // Add a list item
  let name = format!("rust_sdk_example_{}", chrono::Utc::now().timestamp_millis());
  let request = SaveListItemRequest {
    id: None,
    name: name.clone(),
    numeric_value: Some(42.0),
    color: Some("#FF5733".to_string()),
  };

  let response = client.save_list_item(list_id, request).await?;
  println!("Added list item '{}' with ID: {}", name, response.id);

  // Update the list item
  let update_name = format!("updated_{}", name);
  let update_request = SaveListItemRequest {
    id: Some(response.id),
    name: update_name.clone(),
    numeric_value: Some(99.0),
    color: Some("#00FF00".to_string()),
  };

  client.save_list_item(list_id, update_request).await?;
  println!("Updated list item to '{}'", update_name);

  // Delete the list item
  client.delete_list_item(list_id, response.id).await?;
  println!("Deleted list item {}", response.id);

  Ok(())
}
