pub(crate) mod api;
pub(crate) mod common;

use axum::http::StatusCode;
use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};
use serde_json::json;

use common::{test_request_builder::api_versions, TestRequest};

const ENDPOINT_MISSING_API_URI: &str = "/api/none/existent";
const ENDPOINT_MISSING_URI: &str = "/none/existent";

#[tokio::test]
pub(crate) async fn api_uri_not_found_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();

    let response_body = TestRequest::get(ENDPOINT_MISSING_API_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        .send()
        .await
        .assert_status(StatusCode::NOT_FOUND)
        .assert_deserializes_to::<serde_json::Value>();

    let relative_path = ENDPOINT_MISSING_API_URI
        .split_once("/api")
        .expect("missing `/api` in URI")
        .1;

    assert_eq!(
        response_body,
        json!(
            {
                "http_response": {
                    "code": 404,
                    "message": "We could not find the resource you requested."
                },
                "code": 4,
                "description": "",
                "details": {},
                "message": format!("Relative path ({relative_path}) is not a known endpoint resource. Please refer to documentation for list of endpoint resources.")
            }
        )
    );
}

#[tokio::test]
pub(crate) async fn uri_not_found_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();

    let response_body = TestRequest::get(ENDPOINT_MISSING_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        .send()
        .await
        .assert_status(StatusCode::OK);

    let response_body_str = response_body.text();

    assert!(response_body_str.contains("Application error"));
}
