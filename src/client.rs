use std::time::Duration;

use bytes::Bytes;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{OnspringError, Result};

/// Client for interacting with the Onspring API v2.
pub struct OnspringClient {
    http_client: reqwest::Client,
    base_url: String,
    api_key: String,
}

/// Builder for constructing an [`OnspringClient`].
pub struct OnspringClientBuilder {
    base_url: String,
    api_key: String,
    http_client: Option<reqwest::Client>,
    timeout: Option<Duration>,
}

impl OnspringClientBuilder {
    /// Creates a new builder with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: "https://api.onspring.com".to_string(),
            api_key: api_key.into(),
            http_client: None,
            timeout: None,
        }
    }

    /// Sets the base URL for the API.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sets a custom `reqwest::Client` to use for HTTP requests.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Sets the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Builds the [`OnspringClient`].
    pub fn build(self) -> OnspringClient {
        let http_client = self.http_client.unwrap_or_else(|| {
            reqwest::Client::builder()
                .timeout(self.timeout.unwrap_or(Duration::from_secs(120)))
                .build()
                .expect("failed to build HTTP client")
        });

        OnspringClient {
            http_client,
            base_url: self.base_url.trim_end_matches('/').to_string(),
            api_key: self.api_key,
        }
    }
}

impl OnspringClient {
    /// Creates a new [`OnspringClientBuilder`].
    pub fn builder(api_key: impl Into<String>) -> OnspringClientBuilder {
        OnspringClientBuilder::new(api_key)
    }

    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-apikey",
            HeaderValue::from_str(&self.api_key).expect("invalid API key"),
        );
        headers.insert("x-api-version", HeaderValue::from_static("2"));
        headers
    }

    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self
            .http_client
            .request(method, &url)
            .headers(self.default_headers())
            .query(query);

        if let Some(body) = body {
            req = req.json(body);
        }

        let response = req.send().await?;
        let status = response.status();

        if !status.is_success() {
            let message = response
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|v| v.get("message")?.as_str().map(String::from))
                .unwrap_or_default();
            return Err(OnspringError::Api {
                status_code: status.as_u16(),
                message,
            });
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(OnspringError::Serialization)
    }

    pub(crate) async fn request_no_content(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&impl Serialize>,
    ) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self
            .http_client
            .request(method, &url)
            .headers(self.default_headers())
            .query(query);

        if let Some(body) = body {
            req = req.json(body);
        }

        let response = req.send().await?;
        let status = response.status();

        if !status.is_success() {
            let message = response
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|v| v.get("message")?.as_str().map(String::from))
                .unwrap_or_default();
            return Err(OnspringError::Api {
                status_code: status.as_u16(),
                message,
            });
        }

        Ok(())
    }

    pub(crate) async fn request_bytes(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<(StatusCode, HeaderMap, Bytes)> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .request(method, &url)
            .headers(self.default_headers())
            .query(query)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let message = response
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|v| v.get("message")?.as_str().map(String::from))
                .unwrap_or_default();
            return Err(OnspringError::Api {
                status_code: status.as_u16(),
                message,
            });
        }

        let headers = response.headers().clone();
        let bytes = response.bytes().await?;
        Ok((status, headers, bytes))
    }

    pub(crate) async fn request_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .post(&url)
            .headers(self.default_headers())
            .multipart(form)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let message = response
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|v| v.get("message")?.as_str().map(String::from))
                .unwrap_or_default();
            return Err(OnspringError::Api {
                status_code: status.as_u16(),
                message,
            });
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(OnspringError::Serialization)
    }
}
