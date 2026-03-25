use onspring::{OnspringClient, PagingRequest};

#[tokio::main]
async fn main() -> onspring::Result<()> {
  let base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL is required");
  let api_key = std::env::var("SANDBOX_API_KEY").expect("SANDBOX_API_KEY is required");
  let app_id: i32 = std::env::var("TEST_SURVEY_ID")
    .expect("TEST_SURVEY_ID is required")
    .parse()
    .expect("TEST_SURVEY_ID must be an integer");

  let client = OnspringClient::builder(&api_key)
    .base_url(&base_url)
    .build();

  // List reports for an app
  let paging = PagingRequest {
    page_number: 1,
    page_size: 10,
  };
  let response = client.list_reports(app_id, Some(paging)).await?;
  println!("Total reports: {:?}", response.total_records);

  if let Some(items) = &response.items {
    for report in items {
      println!(
        "  Report: {} (id: {}, app_id: {})",
        report.name.as_deref().unwrap_or("N/A"),
        report.id,
        report.app_id,
      );
    }

    // Get report data for the first report
    if let Some(first) = items.first() {
      let data = client.get_report(first.id, None, None).await?;
      println!(
        "\nReport '{}' data:",
        first.name.as_deref().unwrap_or("N/A")
      );
      println!("  Columns: {:?}", data.columns);

      if let Some(rows) = &data.rows {
        println!("  Rows: {}", rows.len());
        for (i, row) in rows.iter().take(3).enumerate() {
          println!(
            "    Row {}: record_id={:?}, cells={:?}",
            i, row.record_id, row.cells
          );
        }
      }
    }
  }

  Ok(())
}
