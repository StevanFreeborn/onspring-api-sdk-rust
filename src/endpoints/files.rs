use reqwest::Method;

use crate::client::OnspringClient;
use crate::error::Result;
use crate::models::{CreatedWithIdResponse, FileInfo, FileResponse, SaveFileRequest};

impl OnspringClient {
  /// Gets a file's metadata information.
  pub async fn get_file_info(
    &self,
    record_id: i32,
    field_id: i32,
    file_id: i32,
  ) -> Result<FileInfo> {
    let path = format!(
      "/Files/recordId/{}/fieldId/{}/fileId/{}",
      record_id, field_id, file_id
    );
    self
      .request(Method::GET, &path, &[], Option::<&()>::None)
      .await
  }

  /// Downloads a file's content.
  pub async fn get_file(
    &self,
    record_id: i32,
    field_id: i32,
    file_id: i32,
  ) -> Result<FileResponse> {
    let path = format!(
      "/Files/recordId/{}/fieldId/{}/fileId/{}/file",
      record_id, field_id, file_id
    );
    let (_status, headers, data) = self.request_bytes(Method::GET, &path, &[]).await?;

    let content_type = headers
      .get("content-type")
      .and_then(|v| v.to_str().ok())
      .map(String::from);

    let file_name = headers
      .get("content-disposition")
      .and_then(|v| v.to_str().ok())
      .and_then(|v| {
        v.split("filename=")
          .nth(1)
          .map(|s| s.trim_matches('"').to_string())
      });

    Ok(FileResponse {
      content_type,
      file_name,
      data,
    })
  }

  /// Uploads a file attachment.
  pub async fn upload_file(&self, request: SaveFileRequest) -> Result<CreatedWithIdResponse> {
    let file_part = reqwest::multipart::Part::bytes(request.file_data)
      .file_name(request.file_name)
      .mime_str(&request.content_type)
      .map_err(|e| crate::error::OnspringError::InvalidArgument(e.to_string()))?;

    let mut form = reqwest::multipart::Form::new()
      .text("RecordId", request.record_id.to_string())
      .text("FieldId", request.field_id.to_string())
      .part("File", file_part);

    if let Some(notes) = request.notes {
      form = form.text("Notes", notes);
    }
    if let Some(date) = request.modified_date {
      form = form.text("ModifiedDate", date.to_rfc3339());
    }

    self.request_multipart("/Files", form).await
  }

  /// Deletes a file attachment.
  pub async fn delete_file(&self, record_id: i32, field_id: i32, file_id: i32) -> Result<()> {
    let path = format!(
      "/Files/recordId/{}/fieldId/{}/fileId/{}",
      record_id, field_id, file_id
    );
    self
      .request_no_content(Method::DELETE, &path, &[], Option::<&()>::None)
      .await
  }
}
