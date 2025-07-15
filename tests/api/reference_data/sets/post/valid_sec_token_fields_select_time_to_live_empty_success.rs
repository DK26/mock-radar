use axum::http::StatusCode;

use serde_json::json;

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use crate::{
    api::reference_data::sets::ENDPOINT_URI,
    common::{test_request_builder::api_versions, TestRequest},
};

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_fields_select_time_to_live_empty_success() {
    let shared_qradar_mock = SharedQRadarMock::default();

    // Mandatory fields
    let element_type = "IP";
    let name = "test_ip_addresses";

    // Optional fields
    let fields = "time_to_live";
    let timeout_type = "FIRST_SEEN";

    let response_body = TestRequest::post(ENDPOINT_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        .query_param("element_type", element_type)
        .query_param("name", name)
        .query_param("fields", fields)
        .query_param("timeout_type", timeout_type)
        .send()
        .await
        .assert_status(StatusCode::CREATED)
        .assert_deserializes_to::<serde_json::Value>();

    assert_eq!(response_body, json!({}));
}
