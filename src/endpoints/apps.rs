use reqwest::Method;

use crate::client::OnspringClient;
use crate::error::Result;
use crate::models::{App, CollectionResponse, PagedResponse, PagingRequest};

impl OnspringClient {
    /// Gets all apps for the current client, with optional pagination.
    pub async fn list_apps(&self, paging: Option<PagingRequest>) -> Result<PagedResponse<App>> {
        let mut query = Vec::new();
        if let Some(p) = paging {
            query.push(("PageNumber", p.page_number.to_string()));
            query.push(("PageSize", p.page_size.to_string()));
        }
        let query_refs: Vec<(&str, String)> = query.iter().map(|(k, v)| (*k, v.clone())).collect();
        self.request(Method::GET, "/Apps", &query_refs, Option::<&()>::None)
            .await
    }

    /// Gets an app by its identifier.
    pub async fn get_app(&self, app_id: i32) -> Result<App> {
        let path = format!("/Apps/id/{}", app_id);
        self.request(Method::GET, &path, &[], Option::<&()>::None)
            .await
    }

    /// Gets up to 100 apps by their identifiers.
    pub async fn batch_get_apps(&self, ids: &[i32]) -> Result<CollectionResponse<App>> {
        self.request(Method::POST, "/Apps/batch-get", &[], Some(&ids))
            .await
    }
}
