use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{extractors::Permissions, SharedQRadarMock};

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn get_reference_data_sets_handler(
    Permissions(authorization_token): Permissions,
    State(shared_qradar_mock): State<SharedQRadarMock>,
    headers: HeaderMap,
) -> impl IntoResponse {
    Html("<h1>Es tut mir leid.. Work in progress</h1>")
}
