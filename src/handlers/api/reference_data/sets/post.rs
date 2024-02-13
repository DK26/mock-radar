use std::time::{SystemTime, UNIX_EPOCH};
use tracing::error;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;

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

#[tracing::instrument(level = "debug", ret, skip(shared_qradar_mock))]
pub(crate) async fn post_reference_data_sets_handler(
    WritePermission(write_permission): WritePermission,
    State(shared_qradar_mock): State<SharedQRadarMock>,
    MaybeQuery(maybe_post_request): MaybeQuery<PostRequest>,
    headers: HeaderMap,
) -> anyhow::Result<Response, Response> {
    let post_request = maybe_post_request.ok_or_else(|| {
        handlers::errors::response::create_unprocessable_entity_query_parameter_missing_response(
            "name",
        )
    })?;

    let name_param = post_request.name.ok_or_else(|| {
        handlers::errors::response::create_unprocessable_entity_query_parameter_missing_response(
            "name",
        )
    })?;

    let element_type_param = post_request.element_type.ok_or_else(|| {
        handlers::errors::response::create_unprocessable_entity_query_parameter_missing_response(
            "element_type",
        )
    })?;

    let mut qradar_mock_write_guard = shared_qradar_mock
        .write()
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE.into_response())?;

    let action_result = qradar_mock_write_guard.add_reference_set(
        write_permission,
        name_param.clone(),
        element_type_param.parse().map_err(|_| {
            handlers::errors::response::create_unprocessable_entity_query_parameter_type_mismatch_response(
                "element_type",
            )
        })?,
    );

    match action_result {
        Ok(_) => {
            let now = SystemTime::now();
            let creation_time = now.duration_since(UNIX_EPOCH).map_err(|e|{
                error!("Did not expect the following error: {e:#?}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            })?.as_millis();
            Ok(
                (
                    StatusCode::CREATED,
                    Json(json!(
                        {
                            "timeout_type": post_request.timeout_type.unwrap_or_else(||String::from("UNKNOWN")),
                            "number_of_elements": 0,
                            "creation_time": creation_time,
                            "name": name_param,
                            "element_type": element_type_param
                        }
                    ))
                ).into_response()
            )
        },
        Err(e) => match e {
            crate::qradar::reference_data::sets::ReferenceSetError::ReferenceSetAlreadyExists(
                name,
            ) => Err((
                StatusCode::CONFLICT,
                Json(json!(
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
                )),
            ).into_response()),
            e => {
                // TODO: Maybe consider a better organization for the ReferenceSet errors, 
                //  due to having a single failure result per specific actions
                // No other error could be produced here.
                error!("Did not expect the following error: {e:#?}");
                Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        },
    }
}
