use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use serde::Deserialize;
use serde_json::json;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use mock_radar::{SharedQRadarMock, REGISTERED_READONLY_SEC_TOKEN, REGISTERED_SEC_TOKEN};

use super::ENDPOINT_URI;

// TODO: Move types to a module of their own
/// This type exists only as workaround to ignore the dynamic value of `creation_time` while validating it.
/// If a better or more proper approach will be found in the future, this may get removed.
#[derive(Debug, Deserialize)]
struct TestPostResponse {
    timeout_type: String,
    number_of_elements: u32,
    #[allow(unused)]
    creation_time: u64, // Assuming successful deserialization implies a valid timestamp
    name: String,
    element_type: String,
}

impl PartialEq for TestPostResponse {
    fn eq(&self, other: &Self) -> bool {
        self.timeout_type == other.timeout_type
            && self.number_of_elements == other.number_of_elements
            && self.name == other.name
            && self.element_type == other.element_type
        // Ignoring `creation_time` in comparison
    }
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_success() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let name: &str = "test_ip_addresses";
    let element_type = "IP";

    let uri = format!("{ENDPOINT_URI}?element_type={element_type}&name={name}");
    let encoded_uri = format!("/{}", urlencoding::encode(&uri));

    let response = router
        .with_state(shared_qradar_mock)
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(encoded_uri)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::CREATED);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");
    let response_body: TestPostResponse = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(
        response_body,
        TestPostResponse {
            timeout_type: "UNKNOWN".to_string(),
            number_of_elements: 0,
            creation_time: 0, // Ignoring `creation_time` in comparison
            name: name.to_string(),
            element_type: element_type.to_string()
        }
    );
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_conflict_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let name: &str = "test_ip_addresses";
    let element_type = "IP";

    let uri = format!("{ENDPOINT_URI}?element_type={element_type}&name={name}");
    let encoded_uri = format!("/{}", urlencoding::encode(&uri));

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(&encoded_uri)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::CREATED);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");
    let response_body: TestPostResponse = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(
        response_body,
        TestPostResponse {
            timeout_type: "UNKNOWN".to_string(),
            number_of_elements: 0,
            creation_time: 0, // Ignoring `creation_time` in comparison
            name: name.to_string(),
            element_type: element_type.to_string()
        }
    );

    let response = router
        .with_state(shared_qradar_mock)
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(urlencoding::encode(&uri).as_ref())
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::CONFLICT);

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
                    "code": 409,
                    "message": "The request could not be completed due to a conflict with the current state of the resource"
                },
                "code": 1004,
                "description": "The reference set could not be created, the name provided is already in use. Please change the name and try again.",
                "details": {},
                "message": format!("The name {name} is already in use")
            }
        )
    );
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_invalid_element_type_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let name: &str = "test_ip_addresses";
    let element_type = "invalid_element";

    let uri = format!("{ENDPOINT_URI}?element_type={element_type}&name={name}");
    let encoded_uri = format!("/{}", urlencoding::encode(&uri));

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(&encoded_uri)
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
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_readonly_sec_token_forbidden_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let name: &str = "test_ip_addresses";
    let element_type = "IP";

    let uri = format!("{ENDPOINT_URI}?element_type={element_type}&name={name}");
    let encoded_uri = format!("/{}", urlencoding::encode(&uri));

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(&encoded_uri)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_READONLY_SEC_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

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
                    "code": 403,
                    "message": "Your account is not authorized to access the requested resource"
                },
                "code": 26,
                "description": "",
                "details": {},
                "message": "User has insufficient capabilities to access this endpoint resource"
            }
        )
    );
}
