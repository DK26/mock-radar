use axum::http::StatusCode;

use serde_json::json;

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use super::TestPostResponse;
use crate::api::reference_data::sets::{post::CreationTime, ENDPOINT_URI};
use crate::common::{test_request_builder::api_versions, TestRequest};

#[tokio::test]
pub(crate) async fn post_reference_set_with_sec_token_conflict_failure() {
    let shared_qradar_mock = SharedQRadarMock::default();

    let name = "test_ip_addresses";
    let element_type = "IP";

    // First request: Create reference set successfully
    let response_body = TestRequest::post(ENDPOINT_URI)
        .with_mock(shared_qradar_mock.clone())
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        .query_param("element_type", element_type)
        .query_param("name", name)
        .send()
        .await
        .assert_status(StatusCode::CREATED)
        .assert_deserializes_to::<TestPostResponse>();

    assert_eq!(
        response_body,
        TestPostResponse {
            timeout_type: "UNKNOWN".to_string(),
            number_of_elements: 0,
            creation_time: CreationTime::default(), // Ignoring `creation_time` in comparison
            name: name.to_string(),
            element_type: element_type.to_string(),
            time_to_live: None
        }
    );

    // Second request: Try to create the same reference set again (should conflict)
    let conflict_response = TestRequest::post(ENDPOINT_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        .query_param("element_type", element_type)
        .query_param("name", name)
        .send()
        .await
        .assert_status(StatusCode::CONFLICT)
        .assert_deserializes_to::<serde_json::Value>();

    assert_eq!(
        conflict_response,
        json!(
            {
                "http_response": {
                    "code": 409,
                    "message": "The request could not be completed due to a conflict with the current state of the resource"
                },
                "code": 1004,
                "description": "The reference set could not be created, the name provided is already in use. Please change the name and try again.",
                "details": {},
                "message": format!("The name {name} is already in use")
            }
        )
    );
}
