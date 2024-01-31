mod common;

use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use mock_radar::{SharedQRadarMock, REGISTERED_TOKEN};
use serde::Deserialize;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

const ENDPOINT_URI: &str = "/api/reference_data/sets";

// TODO: Move types to a module of their own
#[derive(Debug, Deserialize)]
struct TestGetResponse {
    timeout_type: String,
    number_of_elements: u32,
    #[allow(unused)]
    creation_time: u64, // Assuming successful deserialization implies a valid timestamp
    name: String,
    element_type: String,
}

impl PartialEq for TestGetResponse {
    fn eq(&self, other: &Self) -> bool {
        self.timeout_type == other.timeout_type
            && self.number_of_elements == other.number_of_elements
            && self.name == other.name
            && self.element_type == other.element_type
        // Ignoring `creation_time` in comparison
    }
}

#[tokio::test]
pub(crate) async fn get_reference_set() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let response = router
        .with_state(shared_qradar_mock)
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(ENDPOINT_URI)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_token_success() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let name: &str = "test_ip_addresses";
    let element_type = "IP";

    let uri = format!("{ENDPOINT_URI}?element_type={element_type}&name={name}");

    let response = router
        .with_state(shared_qradar_mock)
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(urlencoding::encode(&uri).as_ref())
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let body: TestGetResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body,
        TestGetResponse {
            timeout_type: "UNKNOWN".to_string(),
            number_of_elements: 0,
            creation_time: 0, // Ignoring `creation_time` in comparison
            name: name.to_string(),
            element_type: element_type.to_string()
        }
    )
}

pub(crate) async fn post_reference_set_with_again_with_token_failure() {
    // {
    //     "http_response": {
    //       "code": 409,
    //       "message": "The request could not be completed due to a conflict with the current state of the resource"
    //     },
    //     "code": 1004,
    //     "description": "The reference set could not be created, the name provided is already in use. Please change the name and try again.",
    //     "details": {},
    //     "message": "The name test1234 is already in use"
    //   }
    todo!()
}
