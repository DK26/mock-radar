use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};

use serde_json::json;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use crate::api::reference_data::sets::ENDPOINT_URI;

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_fields_select_time_to_live_empty_success() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    // Mandatory fields
    let element_type = "IP";
    let name = "test_ip_addresses";

    // Optional fields
    let fields = "time_to_live";
    let timeout_type = "FIRST_SEEN";

    let uri = format!(
        "{ENDPOINT_URI}?element_type={}&name={}&fields={}&timeout_type={}",
        urlencoding::encode(element_type),
        urlencoding::encode(name),
        urlencoding::encode(fields),
        urlencoding::encode(timeout_type),
    );

    let response = router
        .with_state(shared_qradar_mock)
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(uri)
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
    let response_body: serde_json::Value = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(response_body, json!({}));
}
