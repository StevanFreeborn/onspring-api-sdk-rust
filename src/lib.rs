/// Rust SDK for the Onspring API v2.
///
/// # Example
///
/// ```no_run
/// use onspring::{OnspringClient, PagingRequest};
///
/// # async fn example() -> onspring::Result<()> {
/// let client = OnspringClient::builder("your-api-key").build();
///
/// // Check connectivity
/// client.ping().await?;
///
/// // List apps
/// let apps = client.list_apps(None).await?;
/// # Ok(())
/// # }
/// ```
pub mod client;
mod endpoints;
pub mod error;
pub mod models;

pub use client::{OnspringClient, OnspringClientBuilder};
pub use error::{OnspringError, Result};
pub use models::*;
