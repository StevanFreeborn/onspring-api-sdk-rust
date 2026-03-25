/// Errors that can occur when using the Onspring API client.
#[derive(Debug, thiserror::Error)]
pub enum OnspringError {
  /// An HTTP transport error occurred.
  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  /// The API returned a non-success status code.
  #[error("API error (status {status_code}): {message}")]
  Api { status_code: u16, message: String },

  /// A serialization or deserialization error occurred.
  #[error("Serialization error: {0}")]
  Serialization(#[from] serde_json::Error),

  /// An invalid argument was provided to an SDK method.
  #[error("Invalid argument: {0}")]
  InvalidArgument(String),
}

/// A type alias for `Result<T, OnspringError>`.
pub type Result<T> = std::result::Result<T, OnspringError>;
