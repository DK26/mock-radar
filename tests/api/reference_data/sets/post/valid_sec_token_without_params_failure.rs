use axum::http::StatusCode;

use serde_json::json;

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use crate::api::reference_data::sets::ENDPOINT_URI;
use crate::common::{test_request_builder::api_versions, TestRequest};

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_without_params_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();

    let response_body = TestRequest::post(ENDPOINT_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        // Intentionally no query parameters to test missing params
        .send()
        .await
        .assert_status(StatusCode::UNPROCESSABLE_ENTITY)
        .assert_deserializes_to::<serde_json::Value>();

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
