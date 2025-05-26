use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};

use serde_json::json;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use crate::api::reference_data::sets::ENDPOINT_URI;

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_fields_selection_single_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

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
                "message": format!("Specified field \"invalid\" is not recognized by this endpoint for media type \"application/json\"")
            }
        )
    );
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_fields_selection_multiple_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let fields = "name, invalid1, invalid2";

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
                "message": "Specified field \"invalid1\" is not recognized by this endpoint for media type \"application/json\""
            }
        )
    );
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_fields_selection_multiple_redundant_failure()
{
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let fields = "name, invalid1, invalid2, name";

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
                "code": 31,
                "description": "",
                "details": {},
                "message": "fields parameter value \"name, invalid1, invalid2, name\" was invalid. Specified field \"name\" was used more than once at the same level"
            }
        ) // TODO: What happens if we put more than 1 double?
    );
}

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_fields_selection_empty_value_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let fields = " ";

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
}
