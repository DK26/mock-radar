use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};

use crate::{extractors::permissions::ReadPermission, SharedQRadarMock};

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn get_reference_data_sets_handler(
    ReadPermission(read_permission): ReadPermission,
    State(shared_qradar_mock): State<SharedQRadarMock>,
    headers: HeaderMap,
) -> impl IntoResponse {
    Html("<h1>Es tut mir leid.. Work in progress</h1>")
}
