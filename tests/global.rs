use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use mock_radar::{SharedQRadarMock, REGISTERED_TOKEN};
use serde_json::json;
use tower::ServiceExt;

const ENDPOINT_MISSING_API_URI: &str = "/api/none/existent";
const ENDPOINT_MISSING_URI: &str = "/none/existent";

#[tokio::test]
pub(crate) async fn api_uri_not_found_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(ENDPOINT_MISSING_API_URI)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");
    let response_body: serde_json::Value = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

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
    let router = mock_radar::create_routes();

    let response = router
        .clone()
        .with_state(shared_qradar_mock.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(ENDPOINT_MISSING_URI)
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header("SEC", REGISTERED_TOKEN)
                .body(Body::empty())
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::OK);

    let response_body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("cannot convert response to Bytes");

    let response_body_str = String::from_utf8_lossy(&response_body_bytes);

    assert!(response_body_str.contains("Application error"));
}
