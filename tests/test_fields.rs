mod common;

use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_get_field_success() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Fields/id/100"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": 100,
        "appId": 1,
        "name": "Text Field",
        "type": "Text",
        "status": "Enabled",
        "isRequired": true,
        "isUnique": false
    })))
    .mount(&mock_server)
    .await;

  let field = client.get_field(100).await.unwrap();
  assert_eq!(field.id, 100);
  assert_eq!(field.app_id, 1);
  assert_eq!(field.name.as_deref(), Some("Text Field"));
  assert!(field.is_required);
  assert!(!field.is_unique);
}

#[tokio::test]
async fn test_get_field_list_type() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Fields/id/200"))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "id": 200,
        "appId": 1,
        "name": "Status",
        "type": "List",
        "status": "Enabled",
        "isRequired": false,
        "isUnique": false,
        "multiplicity": "SingleSelect",
        "listId": 50,
        "values": [
            {
                "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
                "name": "Active",
                "sortOrder": 1,
                "numericValue": 1.0,
                "color": "#00ff00"
            }
        ]
    })))
    .mount(&mock_server)
    .await;

  let field = client.get_field(200).await.unwrap();
  assert_eq!(
    field.multiplicity,
    Some(onspring::Multiplicity::SingleSelect)
  );
  assert_eq!(field.list_id, Some(50));
  let values = field.values.unwrap();
  assert_eq!(values.len(), 1);
  assert_eq!(values[0].name, "Active");
}

#[tokio::test]
async fn test_batch_get_fields() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("POST"))
        .and(path("/Fields/batch-get"))
        .and(body_json(serde_json::json!([1, 2])))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 2,
            "items": [
                {"id": 1, "appId": 1, "name": "Field 1", "type": "Text", "status": "Enabled", "isRequired": false, "isUnique": false},
                {"id": 2, "appId": 1, "name": "Field 2", "type": "Number", "status": "Enabled", "isRequired": true, "isUnique": false}
            ]
        })))
        .mount(&mock_server)
        .await;

  let result = client.batch_get_fields(&[1, 2]).await.unwrap();
  assert_eq!(result.count, Some(2));
  assert_eq!(result.items.unwrap().len(), 2);
}

#[tokio::test]
async fn test_list_fields_for_app() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
        .and(path("/Fields/appId/5"))
        .and(query_param("PageNumber", "1"))
        .and(query_param("PageSize", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "pageNumber": 1,
            "pageSize": 25,
            "totalPages": 1,
            "totalRecords": 3,
            "items": [
                {"id": 10, "appId": 5, "name": "Name", "type": "Text", "status": "Enabled", "isRequired": true, "isUnique": false},
                {"id": 11, "appId": 5, "name": "Email", "type": "Text", "status": "Enabled", "isRequired": false, "isUnique": true},
                {"id": 12, "appId": 5, "name": "Notes", "type": "Text", "status": "Enabled", "isRequired": false, "isUnique": false}
            ]
        })))
        .mount(&mock_server)
        .await;

  let paging = onspring::PagingRequest {
    page_number: 1,
    page_size: 25,
  };
  let result = client.list_fields(5, Some(paging)).await.unwrap();
  assert_eq!(result.total_records, Some(3));
  assert_eq!(result.items.unwrap().len(), 3);
}

#[tokio::test]
async fn test_get_field_not_found() {
  let (mock_server, client) = common::setup().await;

  Mock::given(method("GET"))
    .and(path("/Fields/id/999"))
    .respond_with(ResponseTemplate::new(404))
    .mount(&mock_server)
    .await;

  let result = client.get_field(999).await;
  assert!(result.is_err());
  if let Err(onspring::OnspringError::Api { status_code, .. }) = result {
    assert_eq!(status_code, 404);
  } else {
    panic!("Expected Api error");
  }
}
