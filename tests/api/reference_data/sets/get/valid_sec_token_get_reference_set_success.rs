use axum::http::StatusCode;

use mock_radar::{SharedQRadarMock, REGISTERED_SEC_TOKEN};

use crate::{
    api::reference_data::sets::ENDPOINT_URI,
    common::{test_request_builder::api_versions, TestRequest},
};

#[tokio::test]
pub(crate) async fn get_reference_set_with_sec_token_success() {
    let shared_qradar_mock = SharedQRadarMock::default();

    TestRequest::get(ENDPOINT_URI)
        .with_mock(shared_qradar_mock)
        .content_type(mime::APPLICATION_JSON)
        .accept(mime::APPLICATION_JSON)
        .version(api_versions::V12_0)
        .sec_token(REGISTERED_SEC_TOKEN)
        .send()
        .await
        .assert_status(StatusCode::OK);

    todo!("Complete this test to verify response body and status code");
}
