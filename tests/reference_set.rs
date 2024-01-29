use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use mock_radar::SharedQRadarMock;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

#[tokio::test]
pub(crate) async fn get_reference_set() {
    let shared_state = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let response = router
        .with_state(shared_state)
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/reference_data/sets")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
pub(crate) async fn post_reference_set() {
    let shared_state = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let response = router
        .with_state(shared_state)
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/reference_data/sets")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
