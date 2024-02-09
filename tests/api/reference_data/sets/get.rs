use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use mock_radar::SharedQRadarMock;

use super::ENDPOINT_URI;

#[tokio::test]
pub(crate) async fn get_reference_set_with_sec_token_success() {
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
                .expect("could not build request"),
        )
        .await
        .expect("could not get response");

    assert_eq!(response.status(), StatusCode::OK);
}
