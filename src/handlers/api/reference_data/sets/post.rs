use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{extractors::permissions::Permissions, SharedQRadarMock};

#[derive(Debug, Deserialize)]
pub(crate) struct PostRequest {
    element_type: String,
    name: String,
    fields: Option<String>,
    time_to_live: Option<String>,
    timeout_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct PostResponse {}

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn post_reference_data_sets_handler(
    Permissions(authorization_token): Permissions,
    State(shared_qradar_mock): State<SharedQRadarMock>,
    Query(request): Query<PostRequest>,
    headers: HeaderMap,
) -> anyhow::Result<Json<PostResponse>, Response> {
    let t = shared_qradar_mock.write();
    Html("<h1>Es tut mir leid.. Work in progress</h1>");
    todo!()
}
