use axum::{extract::State, http::HeaderMap};

use crate::SharedQRadarMock;

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn post_sets_handler(
    State(shared_qradar_mock): State<SharedQRadarMock>,
    headers: HeaderMap,
) -> &'static str {
    todo!()
}

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn get_sets_handler(
    State(shared_qradar_mock): State<SharedQRadarMock>,
    headers: HeaderMap,
) -> &'static str {
    todo!()
}
