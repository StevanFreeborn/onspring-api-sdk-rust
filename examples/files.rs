use onspring::{OnspringClient, SaveFileRequest};

#[tokio::main]
async fn main() -> onspring::Result<()> {
  dotenvy::dotenv().ok();
  let base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL is required");
  let api_key = std::env::var("SANDBOX_API_KEY").expect("SANDBOX_API_KEY is required");
  let record_id: i32 = std::env::var("TEST_RECORD")
    .expect("TEST_RECORD is required")
    .parse()
    .expect("TEST_RECORD must be an integer");
  let field_id: i32 = std::env::var("TEST_ATTACHMENT_FIELD")
    .expect("TEST_ATTACHMENT_FIELD is required")
    .parse()
    .expect("TEST_ATTACHMENT_FIELD must be an integer");
  let file_id: i32 = std::env::var("TEST_ATTACHMENT")
    .expect("TEST_ATTACHMENT is required")
    .parse()
    .expect("TEST_ATTACHMENT must be an integer");

  let client = OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build();

  // Get file info
  let info = client.get_file_info(record_id, field_id, file_id).await?;
  println!("File info:");
  println!("  Name: {:?}", info.name);
  println!("  Content type: {:?}", info.content_type);
  println!("  Created: {:?}", info.created_date);
  println!("  Owner: {:?}", info.owner);

  // Download file content
  let file = client.get_file(record_id, field_id, file_id).await?;
  println!("\nDownloaded file:");
  println!("  File name: {:?}", file.file_name);
  println!("  Content type: {:?}", file.content_type);
  println!("  Size: {} bytes", file.data.len());

  // Upload a file
  let request = SaveFileRequest {
    record_id,
    field_id,
    notes: Some("Uploaded from Rust SDK example".to_string()),
    modified_date: Some(chrono::Utc::now()),
    file_name: "example-upload.txt".to_string(),
    file_data: b"Hello from the Rust SDK!".to_vec(),
    content_type: "text/plain".to_string(),
  };

  let uploaded = client.upload_file(request).await?;
  println!("\nUploaded file with ID: {}", uploaded.id);

  // Delete the uploaded file
  client.delete_file(record_id, field_id, uploaded.id).await?;
  println!("Deleted file {}", uploaded.id);

  Ok(())
}
