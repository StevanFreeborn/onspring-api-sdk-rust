use reqwest::Method;

use crate::client::OnspringClient;
use crate::error::Result;
use crate::models::{
    DataFormat, PagedResponse, PagingRequest, ReportData, ReportDataType, ReportInfo,
};

impl OnspringClient {
    /// Gets report data by report ID.
    pub async fn get_report(
        &self,
        report_id: i32,
        data_format: Option<DataFormat>,
        data_type: Option<ReportDataType>,
    ) -> Result<ReportData> {
        let path = format!("/Reports/id/{}", report_id);
        let mut query = Vec::new();
        if let Some(fmt) = data_format {
            query.push(("apiDataFormat", format!("{:?}", fmt)));
        }
        if let Some(dt) = data_type {
            query.push(("dataType", format!("{:?}", dt)));
        }
        let query_refs: Vec<(&str, String)> = query.iter().map(|(k, v)| (*k, v.clone())).collect();
        self.request(Method::GET, &path, &query_refs, Option::<&()>::None)
            .await
    }

    /// Gets a paginated list of reports for a given application.
    pub async fn list_reports(
        &self,
        app_id: i32,
        paging: Option<PagingRequest>,
    ) -> Result<PagedResponse<ReportInfo>> {
        let path = format!("/Reports/appId/{}", app_id);
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
