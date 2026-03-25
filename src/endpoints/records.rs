use reqwest::Method;

use crate::client::OnspringClient;
use crate::error::Result;
use crate::models::{
  BatchDeleteRecordsRequest, BatchGetRecordsRequest, CollectionResponse, DataFormat, PagedResponse,
  PagingRequest, QueryRecordsRequest, Record, SaveRecordRequest, SaveRecordResponse,
};

impl OnspringClient {
  /// Gets a paginated collection of records for a given app.
  pub async fn list_records(
    &self,
    app_id: i32,
    paging: Option<PagingRequest>,
    field_ids: Option<&[i32]>,
    data_format: Option<DataFormat>,
  ) -> Result<PagedResponse<Record>> {
    let path = format!("/Records/appId/{}", app_id);
    let mut query = Vec::new();
    if let Some(p) = paging {
      query.push(("PageNumber", p.page_number.to_string()));
      query.push(("PageSize", p.page_size.to_string()));
    }
    if let Some(ids) = field_ids {
      let ids_str = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",");
      query.push(("fieldIds", ids_str));
    }
    if let Some(fmt) = data_format {
      query.push(("dataFormat", format!("{:?}", fmt)));
    }
    let query_refs: Vec<(&str, String)> = query.iter().map(|(k, v)| (*k, v.clone())).collect();
    self
      .request(Method::GET, &path, &query_refs, Option::<&()>::None)
      .await
  }

  /// Gets a record by its identifier.
  pub async fn get_record(
    &self,
    app_id: i32,
    record_id: i32,
    field_ids: Option<&[i32]>,
    data_format: Option<DataFormat>,
  ) -> Result<Record> {
    let path = format!("/Records/appId/{}/recordId/{}", app_id, record_id);
    let mut query = Vec::new();
    if let Some(ids) = field_ids {
      let ids_str = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",");
      query.push(("fieldIds", ids_str));
    }
    if let Some(fmt) = data_format {
      query.push(("dataFormat", format!("{:?}", fmt)));
    }
    let query_refs: Vec<(&str, String)> = query.iter().map(|(k, v)| (*k, v.clone())).collect();
    self
      .request(Method::GET, &path, &query_refs, Option::<&()>::None)
      .await
  }

  /// Creates or updates a record.
  pub async fn save_record(&self, request: SaveRecordRequest) -> Result<SaveRecordResponse> {
    self
      .request(Method::PUT, "/Records", &[], Some(&request))
      .await
  }

  /// Deletes a record by its identifier.
  pub async fn delete_record(&self, app_id: i32, record_id: i32) -> Result<()> {
    let path = format!("/Records/appId/{}/recordId/{}", app_id, record_id);
    self
      .request_no_content(Method::DELETE, &path, &[], Option::<&()>::None)
      .await
  }

  /// Gets a batch of records.
  pub async fn batch_get_records(
    &self,
    request: BatchGetRecordsRequest,
  ) -> Result<CollectionResponse<Record>> {
    self
      .request(Method::POST, "/Records/batch-get", &[], Some(&request))
      .await
  }

  /// Queries records using a filter expression.
  pub async fn query_records(
    &self,
    request: QueryRecordsRequest,
    paging: Option<PagingRequest>,
  ) -> Result<PagedResponse<Record>> {
    let mut query = Vec::new();
    if let Some(p) = paging {
      query.push(("PageNumber", p.page_number.to_string()));
      query.push(("PageSize", p.page_size.to_string()));
    }
    let query_refs: Vec<(&str, String)> = query.iter().map(|(k, v)| (*k, v.clone())).collect();
    self
      .request(Method::POST, "/Records/Query", &query_refs, Some(&request))
      .await
  }

  /// Deletes a batch of records.
  pub async fn batch_delete_records(&self, request: BatchDeleteRecordsRequest) -> Result<()> {
    self
      .request_no_content(Method::POST, "/Records/batch-delete", &[], Some(&request))
      .await
  }
}
