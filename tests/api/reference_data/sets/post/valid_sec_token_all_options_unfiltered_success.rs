use axum::http::StatusCode;

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use super::TestPostResponse;
use crate::{
    api::reference_data::sets::{post::CreationTime, ENDPOINT_URI},
    common::{test_request_builder::api_versions, TestRequest},
};

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_all_options_unfiltered_success() {
    let shared_qradar_mock = SharedQRadarMock::default();

    // Mandatory fields
    let element_type = "IP";
    let name = "test_ip_addresses";

    // Optional fields
    let time_to_live = "1 years 2 mons 2 days 3 hours 2 mins 32.50 secs";
    let timeout_type = "FIRST_SEEN";

    let response_body = TestRequest::post(ENDPOINT_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        .query_param("element_type", element_type)
        .query_param("name", name)
        .query_param("time_to_live", time_to_live)
        .query_param("timeout_type", timeout_type)
        .send()
        .await
        .assert_status(StatusCode::CREATED)
        .assert_deserializes_to::<TestPostResponse>();

    assert_eq!(
        response_body,
        TestPostResponse {
            timeout_type: timeout_type.to_string(),
            number_of_elements: 0,
            creation_time: CreationTime::default(), // Ignoring `creation_time` in comparison
            name: name.to_string(),
            element_type: element_type.to_string(),
            time_to_live: Some("1 years 2 mons 2 days 3 hours 2 mins 32.50 secs".to_string())
        }
    );
}
