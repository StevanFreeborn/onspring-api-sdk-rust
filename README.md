# Onspring API Rust SDK

[![CI](https://github.com/StevanFreeborn/onspring-api-sdk-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/StevanFreeborn/onspring-api-sdk-rust/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/onspring.svg)](https://crates.io/crates/onspring)
[![docs.rs](https://docs.rs/onspring/badge.svg)](https://docs.rs/onspring)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)

The Rust SDK for the Onspring API is meant to simplify development in Rust for Onspring customers who want to build integrations with their Onspring instance.

**Note:** This is an unofficial SDK for the Onspring API. It was not built in consultation with Onspring Technologies LLC or a member of their development team.

This SDK was developed independently using Onspring's existing [C# SDK](https://github.com/onspring-technologies/onspring-api-sdk), the Onspring API's [swagger page](https://api.onspring.com/swagger/index.html), and [API documentation](https://software.onspring.com/hubfs/Training/Admin%20Guide%20-%20v2%20API.pdf) as the starting point with the intention of making development of integrations done in Rust with an Onspring instance quicker and more convenient.

## Dependencies

### Rust

Requires Rust edition 2021 or later (Rust 1.56+).

### Key Crates

| Crate | Purpose |
|-------|---------|
| [reqwest](https://crates.io/crates/reqwest) | HTTP client for all API requests |
| [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) | JSON serialization and deserialization |
| [tokio](https://crates.io/crates/tokio) | Async runtime |
| [chrono](https://crates.io/crates/chrono) | Date and time types |
| [uuid](https://crates.io/crates/uuid) | UUID types for list item identifiers |
| [thiserror](https://crates.io/crates/thiserror) | Error type derivation |

## Installation

Add the SDK to your project using Cargo:

```sh
cargo add onspring
```

Or add it directly to your `Cargo.toml`:

```toml
[dependencies]
onspring = "0.1"
tokio = { version = "1", features = ["full"] }
```

## API Key

In order to successfully interact with the Onspring API you will need an API key. API keys are obtained by an Onspring user with permissions to at least **Read** API Keys for your instance via the following steps:

1. Login to the Onspring instance.
2. Navigate to **Administration** > **Security** > **API Keys**.
3. On the list page, add a new API Key - this will require **Create** permissions - or click an existing API key to view its details.
4. Click on the **Developer Information** tab.
5. Copy the **X-ApiKey Header** value from this tab.

**Important:**

- An API Key must have a status of `Enabled` in order to make authorized requests.
- Each API Key must have an assigned Role. This role controls the permissions for requests made. If the API Key used does not have sufficient permissions the requests made won't be successful.

### Permission Considerations

You can think of any API Key as another user in your Onspring instance and therefore it is subject to all the same permission considerations as any other user when it comes to its ability to access data in your instance. The API Key you use needs to have all the correct permissions within your instance to access the data requested. Things to think about in this context are `role security`, `content security`, and `field security`.

## Start Coding

### `OnspringClient`

The most common way to use the SDK is to create an `OnspringClient` instance and call its methods. The builder requires an API key and defaults to `https://api.onspring.com` as the base URL.

It is best practice to read these values in from environment variables or a configuration file for both flexibility and security purposes.

Example `.env` file:

```env
API_KEY=000000ffffff000000ffffff/00000000-ffff-0000-ffff-000000000000
BASE_URL=https://api.onspring.com
```

Example constructing `OnspringClient`:

```rust
use onspring::OnspringClient;
use std::env;

let client = OnspringClient::builder(env::var("API_KEY").unwrap())
  .base_url(env::var("BASE_URL").unwrap())
  .build();
```

### Client Configuration

By default when you construct an instance of the `OnspringClient` a new `reqwest::Client` will also be created with a 120-second timeout. You can customize the timeout or provide your own `reqwest::Client`:

```rust
use onspring::OnspringClient;
use std::time::Duration;

// Custom timeout
let client = OnspringClient::builder("your-api-key")
  .timeout(Duration::from_secs(30))
  .build();

// Custom reqwest client
let http_client = reqwest::Client::builder()
  .timeout(Duration::from_secs(60))
  .build()
  .unwrap();

let client = OnspringClient::builder("your-api-key")
  .http_client(http_client)
  .build();
```

### Error Handling

All client methods return `onspring::Result<T>`, which is an alias for `Result<T, OnspringError>`. The `OnspringError` enum has the following variants:

- `Http` - An HTTP transport error occurred (network issues, timeouts, etc.).
- `Api { status_code, message }` - The API returned a non-success status code.
- `Serialization` - A JSON serialization or deserialization error occurred.
- `InvalidArgument` - An invalid argument was provided to an SDK method.

```rust
match client.get_app(130).await {
  Ok(app) => println!("App: {}", app.name.unwrap_or_default()),
  Err(onspring::OnspringError::Api { status_code, message }) => {
    eprintln!("API error {}: {}", status_code, message);
  }
  Err(e) => eprintln!("Error: {}", e),
}
```

### Full API Documentation

You may wish to refer to the full [Onspring API documentation](https://software.onspring.com/hubfs/Training/Admin%20Guide%20-%20v2%20API.pdf) when determining which values to pass as parameters to some of the `OnspringClient` methods. There is also a [swagger page](https://api.onspring.com/swagger/index.html) that you can use for making exploratory requests.

## Runnable Examples

The `examples/` directory contains runnable examples that demonstrate each endpoint group against a real Onspring instance. They load configuration from a `.env` file in the project root.

Create a `.env` file with your test instance details:

```env
API_BASE_URL=https://your-instance.onspring.com
SANDBOX_API_KEY=your-api-key
TEST_APP_ID=123
TEST_SURVEY_ID=456
TEST_TEXT_FIELD=789
TEST_RECORD=1
TEST_ATTACHMENT_FIELD=101
TEST_ATTACHMENT=202
TEST_IMAGE_FIELD=303
TEST_IMAGE=404
TEST_LIST_ID=505
```

Then run any example:

```sh
cargo run --example ping
cargo run --example apps
cargo run --example fields
cargo run --example records
cargo run --example files
cargo run --example lists
cargo run --example reports
```

## Code Examples

Note the following code snippets assume you've already instantiated an `OnspringClient` as shown in the [OnspringClient](#onspringclient) section and that the code is running within an async context.

### Connectivity

#### Verify connectivity

```rust
client.ping().await?;
println!("Connected to Onspring API");
```

### Apps

#### Get Apps

Returns a paged collection of apps and/or surveys that can be paged through. By default the page size is 50 and page number is 1.

```rust
let res = client.list_apps(None).await?;
let apps = res.items.unwrap_or_default();

for app in &apps {
  println!("{:?}", app);
}
```

You can set your own page size and page number (max is 1,000) as well.

```rust
use onspring::PagingRequest;

let paging = PagingRequest { page_number: 1, page_size: 10 };
let res = client.list_apps(Some(paging)).await?;
let apps = res.items.unwrap_or_default();

for app in &apps {
  println!("{:?}", app);
}
```

#### Get App By Id

Returns an Onspring app or survey according to provided id.

```rust
let app = client.get_app(130).await?;
println!("{:?}", app);
```

#### Get Apps By Ids

Returns a collection of Onspring apps and/or surveys according to provided ids.

```rust
let res = client.batch_get_apps(&[130, 131]).await?;
let apps = res.items.unwrap_or_default();

for app in &apps {
  println!("{:?}", app);
}
```

### Fields

#### Get Field By Id

Returns an Onspring field according to provided id.

```rust
let field = client.get_field(4793).await?;
println!("{:?}", field);
```

#### Get Fields By Ids

Returns a collection of Onspring fields according to provided ids.

```rust
let res = client.batch_get_fields(&[4793, 4801]).await?;
let fields = res.items.unwrap_or_default();

for field in &fields {
  println!("{:?}", field);
}
```

#### Get Fields By App Id

Returns a paged collection of fields that can be paged through. By default the page size is 50 and page number is 1.

```rust
let res = client.list_fields(132, None).await?;
let fields = res.items.unwrap_or_default();

for field in &fields {
  println!("{:?}", field);
}
```

You can set your own page size and page number (max is 1,000) as well.

```rust
use onspring::PagingRequest;

let paging = PagingRequest { page_number: 1, page_size: 10 };
let res = client.list_fields(132, Some(paging)).await?;
let fields = res.items.unwrap_or_default();

for field in &fields {
  println!("{:?}", field);
}
```

### Files

#### Get File Info By Id

Returns the Onspring file's metadata.

```rust
let file_info = client.get_file_info(1, 4806, 909).await?;
println!("{:?}", file_info);
```

#### Get File By Id

Returns the file itself.

```rust
use std::fs;

let file = client.get_file(1, 4806, 909).await?;

println!("Content-Type: {:?}", file.content_type);
println!("File Name: {:?}", file.file_name);

if let Some(name) = &file.file_name {
  fs::write(name, &file.data)?;
}
```

#### Save File

```rust
use onspring::models::file::SaveFileRequest;

let request = SaveFileRequest {
  record_id: 1,
  field_id: 4806,
  notes: Some("notes".to_string()),
  modified_date: None,
  file_name: "test-attachment.txt".to_string(),
  file_data: std::fs::read("test-attachment.txt")?,
  content_type: "text/plain".to_string(),
};

let res = client.upload_file(request).await?;
println!("File ID: {}", res.id);
```

#### Delete File By Id

```rust
client.delete_file(1, 4806, 1505).await?;
println!("File deleted");
```

### Lists

#### Add Or Update List Value

To add a list value don't provide an id value.

```rust
use onspring::models::list::SaveListItemRequest;

let request = SaveListItemRequest {
  id: None,
  name: "New Value".to_string(),
  numeric_value: Some(1.0),
  color: Some("#000000".to_string()),
};

let res = client.save_list_item(638, request).await?;
println!("Item ID: {}", res.id);
```

To update a list value provide an id value.

```rust
use onspring::models::list::SaveListItemRequest;
use uuid::Uuid;

let item_id: Uuid = "35c79a46-04b8-4069-bbc1-161a175f962c".parse().unwrap();

let request = SaveListItemRequest {
  id: Some(item_id),
  name: "Updated Value".to_string(),
  numeric_value: Some(1.0),
  color: Some("#000000".to_string()),
};

let res = client.save_list_item(638, request).await?;
println!("Item ID: {}", res.id);
```

#### Delete List Value

```rust
use uuid::Uuid;

let item_id: Uuid = "35c79a46-04b8-4069-bbc1-161a175f962c".parse().unwrap();
client.delete_list_item(638, item_id).await?;
println!("List item deleted");
```

### Records

#### Get Records By App Id

Returns a paged collection of records that can be paged through. By default the page size is 50 and page number is 1.

```rust
let res = client.list_records(130, None, None, None).await?;
let records = res.items.unwrap_or_default();

for record in &records {
  println!("{:?}", record);
}
```

You can set your own page size and page number (max is 1,000) as well. In addition to specifying what field values to return and in what format (Raw vs. Formatted) to return them.

```rust
use onspring::{DataFormat, PagingRequest};

let paging = PagingRequest { page_number: 1, page_size: 10 };
let res = client.list_records(
  130,
  Some(paging),
  Some(&[4804]),
  Some(DataFormat::Raw),
).await?;
let records = res.items.unwrap_or_default();

for record in &records {
  println!("{:?}", record);
}
```

#### Get Record By Id

Returns an Onspring record based on the provided app and record ids.

```rust
let record = client.get_record(130, 1, None, None).await?;
println!("{:?}", record);
```

You can also specify what field values to return and in what format (Raw vs. Formatted) to return them.

```rust
use onspring::DataFormat;

let record = client.get_record(130, 1, Some(&[4804]), Some(DataFormat::Raw)).await?;
println!("{:?}", record);
```

#### Get Records By Ids

Returns a collection of Onspring records based on the provided app id and record ids.

```rust
use onspring::models::record::BatchGetRecordsRequest;

let request = BatchGetRecordsRequest {
  app_id: 130,
  record_ids: vec![1, 2],
  field_ids: None,
  data_format: None,
};

let res = client.batch_get_records(request).await?;
let records = res.items.unwrap_or_default();

for record in &records {
  println!("{:?}", record);
}
```

You can also specify what field values to return and in what format (Raw vs. Formatted) to return them.

```rust
use onspring::{DataFormat, models::record::BatchGetRecordsRequest};

let request = BatchGetRecordsRequest {
  app_id: 130,
  record_ids: vec![1, 2],
  field_ids: Some(vec![4804]),
  data_format: Some(DataFormat::Formatted),
};

let res = client.batch_get_records(request).await?;
let records = res.items.unwrap_or_default();

for record in &records {
  println!("{:?}", record);
}
```

#### Query Records

Returns a paged collection of records based on a criteria that can be paged through. By default the page size is 50 and page number is 1.

```rust
use onspring::models::record::QueryRecordsRequest;

let request = QueryRecordsRequest {
  app_id: 130,
  filter: "not (4745 eq 0)".to_string(),
  field_ids: None,
  data_format: None,
};

let res = client.query_records(request, None).await?;
let records = res.items.unwrap_or_default();

for record in &records {
  println!("{:?}", record);
}
```

You can set your own page size and page number (max is 1,000) as well. In addition to specifying what field values to return and in what format (Raw vs. Formatted) to return them.

```rust
use onspring::{DataFormat, PagingRequest, models::record::QueryRecordsRequest};

let request = QueryRecordsRequest {
  app_id: 130,
  filter: "not (4745 eq 0)".to_string(),
  field_ids: Some(vec![4804]),
  data_format: Some(DataFormat::Formatted),
};

let paging = PagingRequest { page_number: 1, page_size: 10 };
let res = client.query_records(request, Some(paging)).await?;
let records = res.items.unwrap_or_default();

for record in &records {
  println!("{:?}", record);
}
```

For further details on constructing the `filter` parameter please refer to the [documentation](https://software.onspring.com/hubfs/Training/Admin%20Guide%20-%20v2%20API.pdf) for the Onspring API.

#### Add or Update A Record

You can add a record by not providing a record id value. If successful will return the id of the added record.

```rust
use std::collections::HashMap;
use onspring::models::record::SaveRecordRequest;

let mut fields = HashMap::new();
fields.insert("4804".to_string(), serde_json::json!("Test"));

let request = SaveRecordRequest {
  app_id: 130,
  record_id: None,
  fields,
};

let res = client.save_record(request).await?;
println!("New Record ID: {}", res.id);
```

You can update a record by providing its id. If successful will return the id of record updated.

```rust
use std::collections::HashMap;
use onspring::models::record::SaveRecordRequest;

let mut fields = HashMap::new();
fields.insert("4804".to_string(), serde_json::json!("Updated"));

let request = SaveRecordRequest {
  app_id: 130,
  record_id: Some(607),
  fields,
};

let res = client.save_record(request).await?;
println!("Updated Record ID: {}", res.id);
```

#### Delete Record By Id

Delete an individual record based upon its id.

```rust
client.delete_record(130, 607).await?;
println!("Record deleted");
```

#### Delete Records By Ids

Delete a batch of records based upon their ids.

```rust
use onspring::models::record::BatchDeleteRecordsRequest;

let request = BatchDeleteRecordsRequest {
  app_id: 130,
  record_ids: vec![608, 609],
};

client.batch_delete_records(request).await?;
println!("Records deleted");
```

### Reports

#### Get Report By Id

Returns the report for the provided id.

```rust
let report = client.get_report(408, None, None).await?;
println!("{:?}", report);
```

You can also specify the format of the data in the report as well as whether you are requesting the report's data or its chart data.

```rust
use onspring::{DataFormat, ReportDataType};

let report = client.get_report(
  409,
  Some(DataFormat::Formatted),
  Some(ReportDataType::ChartData),
).await?;
println!("{:?}", report);
```

#### Get Reports By App Id

Returns a paged collection of reports that can be paged through. By default the page size is 50 and page number is 1.

```rust
let res = client.list_reports(130, None).await?;
let reports = res.items.unwrap_or_default();

for report in &reports {
  println!("{:?}", report);
}
```

You can set your own page size and page number (max is 1,000) as well.

```rust
use onspring::PagingRequest;

let paging = PagingRequest { page_number: 1, page_size: 10 };
let res = client.list_reports(130, Some(paging)).await?;
let reports = res.items.unwrap_or_default();

for report in &reports {
  println!("{:?}", report);
}
```
