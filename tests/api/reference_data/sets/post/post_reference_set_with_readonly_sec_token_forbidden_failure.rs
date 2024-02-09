use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};

use serde_json::json;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use mock_radar::{SharedQRadarMock, REGISTERED_READONLY_SEC_TOKEN};

use crate::api::reference_data::sets::ENDPOINT_URI;

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
