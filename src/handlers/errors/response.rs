use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;

/// Creates a 422 error response
pub(crate) fn create_unprocessable_entity_response(bad_field_name: &str) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
        "http_response": {
            "code": 422,
            "message": "The request was well-formed but was unable to be followed due to semantic errors"
        },
        "code": 8,
        "description": "",
        "details": {},
        "message": format!("Missing required parameter \"{bad_field_name}\" from query parameters")
    }))).into_response()
}
