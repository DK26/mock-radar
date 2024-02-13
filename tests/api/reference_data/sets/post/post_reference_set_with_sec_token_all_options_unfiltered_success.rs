use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};

use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use super::TestPostResponse;
use crate::api::reference_data::sets::ENDPOINT_URI;

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_all_options_unfiltered_success() {
    let shared_qradar_mock = SharedQRadarMock::default();
    let router = mock_radar::create_routes();

    // Mandatory fields
    let element_type = "IP";
    let name = "test_ip_addresses";

    // Optional fields
    let time_to_live = "1 year 2 months 2 days 3 hours 2 minutes 32.5 seconds";
    let timeout_type = "FIRST_SEEN";

    let uri = format!(
        "{ENDPOINT_URI}?element_type={}&name={}&time_to_live={}&timeout_type={}",
        urlencoding::encode(element_type),
        urlencoding::encode(name),
        urlencoding::encode(time_to_live),
        urlencoding::encode(timeout_type)
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
    let response_body: TestPostResponse = serde_json::from_slice(&response_body_bytes)
        .expect("cannot deserialize response from Bytes");

    assert_eq!(
        response_body,
        TestPostResponse {
            timeout_type: timeout_type.to_string(),
            number_of_elements: 0,
            creation_time: 0, // Ignoring `creation_time` in comparison
            name: name.to_string(),
            element_type: element_type.to_string(),
            time_to_live: Some("1 years 2 mons 2 days 3 hours 2 mins 32.50 secs".to_string())
        }
    );
}
