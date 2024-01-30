use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use mock_radar::SharedQRadarMock;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

pub const REGISTERED_TOKEN: &str = "d6391576-55d3-4c44-85d8-5665b0d2336f";

const ENDPOINT_REFERENCE_SET: &str = "/reference_data/sets";

#[tokio::test]
pub(crate) async fn get_reference_set() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let response = router
        .with_state(shared_qradar_mock)
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(ENDPOINT_REFERENCE_SET)
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
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    let name: &str = "test_ip_addresses";
    let element_type = "IP";

    let uri = format!("{ENDPOINT_REFERENCE_SET}?element_type={element_type}&name={name}");

    let response = router
        .with_state(shared_qradar_mock)
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(urlencoding::encode(&uri).as_ref())
                .header("Version", "12.0")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::ACCEPT, mime::APPLICATION_JSON.as_ref())
                .header(
                    http::header::AUTHORIZATION,
                    format!("Bearer {REGISTERED_TOKEN}"),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    /*
    RESPONSE BODY:
        {
            "timeout_type": "UNKNOWN",
            "number_of_elements": 0,
            "creation_time": 1706655294903,
            "name": "test1234",
            "element_type": "IP"
        }
    */
}
