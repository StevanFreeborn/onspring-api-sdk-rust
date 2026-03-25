use onspring::OnspringClient;

#[tokio::main]
async fn main() -> onspring::Result<()> {
  let base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL is required");
  let api_key = std::env::var("SANDBOX_API_KEY").expect("SANDBOX_API_KEY is required");

  let client = OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build();

  client.ping().await?;
  println!("Connected to the Onspring API successfully.");

  Ok(())
}
