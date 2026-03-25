use reqwest::Method;

use crate::client::OnspringClient;
use crate::error::Result;
use crate::models::{CollectionResponse, Field, PagedResponse, PagingRequest};

impl OnspringClient {
    /// Gets a field by its identifier.
    pub async fn get_field(&self, field_id: i32) -> Result<Field> {
        let path = format!("/Fields/id/{}", field_id);
        self.request(Method::GET, &path, &[], Option::<&()>::None)
            .await
    }

    /// Gets up to 100 fields by their identifiers.
    pub async fn batch_get_fields(&self, ids: &[i32]) -> Result<CollectionResponse<Field>> {
        self.request(Method::POST, "/Fields/batch-get", &[], Some(&ids))
            .await
    }

    /// Gets a paginated list of fields for a given application.
    pub async fn list_fields(
        &self,
        app_id: i32,
        paging: Option<PagingRequest>,
    ) -> Result<PagedResponse<Field>> {
        let path = format!("/Fields/appId/{}", app_id);
        let mut query = Vec::new();
        if let Some(p) = paging {
            query.push(("PageNumber", p.page_number.to_string()));
            query.push(("PageSize", p.page_size.to_string()));
        }
        let query_refs: Vec<(&str, String)> = query.iter().map(|(k, v)| (*k, v.clone())).collect();
        self.request(Method::GET, &path, &query_refs, Option::<&()>::None)
            .await
    }
}
