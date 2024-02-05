use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};

use crate::{extractors::Permissions, SharedQRadarMock};

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn post_handler(
    State(shared_qradar_mock): State<SharedQRadarMock>,
    headers: HeaderMap,
) -> impl IntoResponse {
    Html("<h1>Es tut mir leid.. Work in progress</h1>")
}

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn get_handler(
    State(shared_qradar_mock): State<SharedQRadarMock>,
    Permissions(authorization_token): Permissions,
    headers: HeaderMap,
) -> impl IntoResponse {
    Html("<h1>Es tut mir leid.. Work in progress</h1>")
}
