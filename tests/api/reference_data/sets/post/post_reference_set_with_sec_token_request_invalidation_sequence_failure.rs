use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};

use serde_json::json;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use crate::api::reference_data::sets::ENDPOINT_URI;

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_request_invalidation_sequence_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    // Sequence 1

    // Mandatory fields
    let element_type = "invalid_element";
    let name = "test_ip_addresses";

    // Optional fields
    let fields = "non_existent";
    let time_to_live = "invalid time";
    let timeout_type = "INVALID_TYPE";

    let uri = format!(
        "{ENDPOINT_URI}?element_type={}&name={}&fields={}&time_to_live={}&timeout_type={}",
        urlencoding::encode(element_type),
        urlencoding::encode(name),
        urlencoding::encode(fields),
        urlencoding::encode(time_to_live),
        urlencoding::encode(timeout_type),
    );

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(&uri)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");
    let response_body: serde_json::Value = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(
        response_body,
        json!(
            {
                "http_response": {
                  "code": 422,
                  "message": "The request was well-formed but was unable to be followed due to semantic errors"
                },
                "code": 30,
                "description": "",
                "details": {},
                "message": format!("Specified field \"{fields}\" is not recognized by this endpoint for media type \"application/json\"")
              }
        )
    );

    // Sequence 2

    // Mandatory fields
    let element_type = "invalid_element";
    let name = "test_ip_addresses";

    // Optional fields
    let fields = "time_to_live";
    let time_to_live = "invalid time";
    let timeout_type = "INVALID_TYPE";

    let uri = format!(
        "{ENDPOINT_URI}?element_type={}&name={}&fields={}&time_to_live={}&timeout_type={}",
        urlencoding::encode(element_type),
        urlencoding::encode(name),
        urlencoding::encode(fields),
        urlencoding::encode(time_to_live),
        urlencoding::encode(timeout_type),
    );

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(&uri)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");
    let response_body: serde_json::Value = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(
        response_body,
        json!(
            {
            "http_response": {
                "code": 422,
                "message": "The request was well-formed but was unable to be followed due to semantic errors"
            },
            "code": 11,
            "description": "",
            "details": {},
            "message": "Failed to transform user query parameter \"element_type\" with a content type of \"TEXT_PLAIN\""
            }
        )
    );

    // Sequence 3

    // Mandatory fields
    let element_type = "IP";
    let name = "test_ip_addresses";

    // Optional fields
    let fields = "time_to_live";
    let time_to_live = "invalid time";
    let timeout_type = "INVALID_TYPE";

    let uri = format!(
        "{ENDPOINT_URI}?element_type={}&name={}&fields={}&time_to_live={}&timeout_type={}",
        urlencoding::encode(element_type),
        urlencoding::encode(name),
        urlencoding::encode(fields),
        urlencoding::encode(time_to_live),
        urlencoding::encode(timeout_type),
    );

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(&uri)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");
    let response_body: serde_json::Value = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(
        response_body,
        json!(
            {
                "http_response": {
                  "code": 422,
                  "message": "The request was well-formed but was unable to be followed due to semantic errors"
                },
                "code": 11,
                "description": "",
                "details": {},
                "message": "Failed to transform user query parameter \"timeout_type\" with a content type of \"TEXT_PLAIN\""
              }
        )
    );

    // Sequence 4

    // Mandatory fields
    let element_type = "IP";
    let name = "test_ip_addresses";

    // Optional fields
    let fields = "time_to_live";
    let time_to_live = "invalid time";
    let timeout_type = "LAST_SEEN";

    let uri = format!(
        "{ENDPOINT_URI}?element_type={}&name={}&fields={}&time_to_live={}&timeout_type={}",
        urlencoding::encode(element_type),
        urlencoding::encode(name),
        urlencoding::encode(fields),
        urlencoding::encode(time_to_live),
        urlencoding::encode(timeout_type),
    );

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(&uri)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");
    let response_body: serde_json::Value = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(
        response_body,
        json!(
            {
                "http_response": {
                  "code": 422,
                  "message": "The request was well-formed but was unable to be followed due to semantic errors"
                },
                "code": 1005,
                "description": "A request parameter is not valid",
                "details": {},
                "message": format!("Invalid time to live interval {time_to_live}")
            }
        )
    );
}
