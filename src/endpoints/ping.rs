use reqwest::Method;

use crate::client::OnspringClient;
use crate::error::Result;

impl OnspringClient {
    /// Checks if the Onspring API is reachable.
    pub async fn ping(&self) -> Result<()> {
        self.request_no_content(Method::GET, "/Ping", &[], Option::<&()>::None)
            .await
    }
}
