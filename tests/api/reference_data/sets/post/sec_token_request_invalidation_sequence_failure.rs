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

    // Optional fields (default = selects all fields)
    let fields = "invalid";

    let uri = format!("{ENDPOINT_URI}?fields={}", urlencoding::encode(fields));

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

    // `name` is a field that exists, but not yet provided
    let fields = "name";

    let uri = format!("{ENDPOINT_URI}?fields={}", urlencoding::encode(fields));

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
                "code": 8,
                "description": "",
                "details": {},
                "message": "Missing required parameter \"name\" from query parameters"
            }
        )
    );

    // Sequence 3

    // Mandatory fields
    let name = "some_name";

    // Optional fields
    let fields = "name";

    let uri = format!(
        "{ENDPOINT_URI}?fields={}&name={}",
        urlencoding::encode(fields),
        urlencoding::encode(name),
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
                "code": 8,
                "description": "",
                "details": {},
                "message": "Missing required parameter \"element_type\" from query parameters"
            }
        )
    );

    // Sequence 4

    // Mandatory fields
    let name = "some_name";
    let element_type = "invalid";

    // Optional fields
    let fields = "name";

    let uri = format!(
        "{ENDPOINT_URI}?fields={}&name={}&element_type={}",
        urlencoding::encode(fields),
        urlencoding::encode(name),
        urlencoding::encode(element_type),
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

    // Sequence 5

    // Mandatory fields
    let element_type = "IP";
    let name = "some_name";

    // Optional fields
    let fields = "name";
    let timeout_type = "invalid";

    let uri = format!(
        "{ENDPOINT_URI}?fields={}&name={}&element_type={}&timeout_type={}",
        urlencoding::encode(fields),
        urlencoding::encode(name),
        urlencoding::encode(element_type),
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

    // Sequence 6

    // Mandatory fields
    let element_type = "IP";
    let name = "some_name";

    // Optional fields
    let fields = "name";
    let timeout_type = "LAST_SEEN";
    let time_to_live = "invalid";

    let uri = format!(
        "{ENDPOINT_URI}?fields={}&name={}&element_type={}&timeout_type={}&time_to_live={}",
        urlencoding::encode(fields),
        urlencoding::encode(name),
        urlencoding::encode(element_type),
        urlencoding::encode(timeout_type),
        urlencoding::encode(time_to_live),
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
