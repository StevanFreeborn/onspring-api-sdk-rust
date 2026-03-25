use reqwest::Method;
use uuid::Uuid;

use crate::client::OnspringClient;
use crate::error::Result;
use crate::models::{SaveListItemRequest, SaveListItemResponse};

impl OnspringClient {
    /// Creates or updates a list item in the specified list.
    pub async fn save_list_item(
        &self,
        list_id: i32,
        request: SaveListItemRequest,
    ) -> Result<SaveListItemResponse> {
        let path = format!("/Lists/id/{}/items", list_id);
        self.request(Method::PUT, &path, &[], Some(&request)).await
    }

    /// Deletes a list item from the specified list.
    pub async fn delete_list_item(&self, list_id: i32, item_id: Uuid) -> Result<()> {
        let path = format!("/Lists/id/{}/itemId/{}", list_id, item_id);
        self.request_no_content(Method::DELETE, &path, &[], Option::<&()>::None)
            .await
    }
}
