use axum::{extract::State, http::HeaderMap, response::Response, Json};
use serde::{Deserialize, Serialize};

use crate::{
    extractors::{maybe_query::MaybeQuery, permissions::WritePermission},
    handlers, SharedQRadarMock,
};

#[derive(Debug, Deserialize)]
pub(crate) struct PostRequest {
    element_type: Option<String>,
    name: Option<String>,
    fields: Option<String>,
    time_to_live: Option<String>,
    timeout_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct PostResponse {}

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn post_reference_data_sets_handler(
    WritePermission(write_permission): WritePermission,
    State(shared_qradar_mock): State<SharedQRadarMock>,
    MaybeQuery(maybe_post_request): MaybeQuery<PostRequest>,
    headers: HeaderMap,
) -> anyhow::Result<Json<PostResponse>, Response> {
    let post_request = maybe_post_request
        .ok_or_else(|| handlers::errors::response::create_unprocessable_entity_response("name"))?;

    let name_param = post_request
        .name
        .ok_or_else(|| handlers::errors::response::create_unprocessable_entity_response("name"))?;

    let element_type_param = post_request.element_type.ok_or_else(|| {
        handlers::errors::response::create_unprocessable_entity_response("element_type")
    })?;

    let t = shared_qradar_mock.write();
    todo!();
}
