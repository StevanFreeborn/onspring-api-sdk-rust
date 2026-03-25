mod common;

use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_get_file_info() {
    let (mock_server, client) = common::setup().await;

    Mock::given(method("GET"))
        .and(path("/Files/recordId/1/fieldId/2/fileId/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "type": "Attachment",
            "contentType": "application/pdf",
            "name": "document.pdf",
            "createdDate": "2024-01-15T10:30:00Z",
            "modifiedDate": "2024-01-15T10:30:00Z",
            "owner": "admin",
            "notes": "Important doc",
            "fileHref": "/Files/recordId/1/fieldId/2/fileId/3/file"
        })))
        .mount(&mock_server)
        .await;

    let info = client.get_file_info(1, 2, 3).await.unwrap();
    assert_eq!(info.name.as_deref(), Some("document.pdf"));
    assert_eq!(info.content_type.as_deref(), Some("application/pdf"));
    assert_eq!(info.owner.as_deref(), Some("admin"));
}

#[tokio::test]
async fn test_get_file_content() {
    let (mock_server, client) = common::setup().await;

    let file_bytes = b"hello file content";
    Mock::given(method("GET"))
        .and(path("/Files/recordId/1/fieldId/2/fileId/3/file"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(file_bytes.to_vec())
                .insert_header("content-type", "application/pdf")
                .insert_header("content-disposition", "attachment; filename=\"test.pdf\""),
        )
        .mount(&mock_server)
        .await;

    let response = client.get_file(1, 2, 3).await.unwrap();
    assert_eq!(response.data.as_ref(), file_bytes);
    assert_eq!(response.content_type.as_deref(), Some("application/pdf"));
    assert_eq!(response.file_name.as_deref(), Some("test.pdf"));
}

#[tokio::test]
async fn test_upload_file() {
    let (mock_server, client) = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/Files"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 42
        })))
        .mount(&mock_server)
        .await;

    let request = onspring::models::file::SaveFileRequest {
        record_id: 1,
        field_id: 2,
        notes: Some("test notes".to_string()),
        modified_date: None,
        file_name: "test.txt".to_string(),
        file_data: b"file content".to_vec(),
        content_type: "text/plain".to_string(),
    };
    let result = client.upload_file(request).await.unwrap();
    assert_eq!(result.id, 42);
}

#[tokio::test]
async fn test_delete_file() {
    let (mock_server, client) = common::setup().await;

    Mock::given(method("DELETE"))
        .and(path("/Files/recordId/1/fieldId/2/fileId/3"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let result = client.delete_file(1, 2, 3).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_file_info_not_found() {
    let (mock_server, client) = common::setup().await;

    Mock::given(method("GET"))
        .and(path("/Files/recordId/1/fieldId/2/fileId/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "File not found"
        })))
        .mount(&mock_server)
        .await;

    let result = client.get_file_info(1, 2, 999).await;
    assert!(result.is_err());
    if let Err(onspring::OnspringError::Api {
        status_code,
        message,
    }) = result
    {
        assert_eq!(status_code, 404);
        assert_eq!(message, "File not found");
    } else {
        panic!("Expected Api error");
    }
}
