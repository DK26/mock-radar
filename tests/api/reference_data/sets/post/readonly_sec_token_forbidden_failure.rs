use axum::http::StatusCode;

use serde_json::json;

use mock_radar::{SharedQRadarMock, REGISTERED_READONLY_SEC_TOKEN};

use crate::api::reference_data::sets::ENDPOINT_URI;
use crate::common::{test_request_builder::api_versions, TestRequest};

#[tokio::test]
pub(crate) async fn post_reference_set_with_readonly_sec_token_forbidden_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();

    let name = "test_ip_addresses";
    let element_type = "IP";

    let response_body = TestRequest::post(ENDPOINT_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_READONLY_SEC_TOKEN)
        .query_param("element_type", element_type)
        .query_param("name", name)
        .send()
        .await
        .assert_status(StatusCode::FORBIDDEN)
        .assert_deserializes_to::<serde_json::Value>();

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
