use serde::Deserialize;

/// Parameters for paginated API requests.
#[derive(Debug, Clone)]
pub struct PagingRequest {
    pub page_number: i32,
    pub page_size: i32,
}

impl Default for PagingRequest {
    fn default() -> Self {
        Self {
            page_number: 1,
            page_size: 50,
        }
    }
}

/// A paginated response from the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedResponse<T> {
    pub page_number: Option<i32>,
    pub page_size: Option<i32>,
    pub total_pages: Option<i32>,
    pub total_records: Option<i32>,
    pub items: Option<Vec<T>>,
}

/// A collection response from the API (non-paginated).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionResponse<T> {
    pub count: Option<i32>,
    pub items: Option<Vec<T>>,
}
