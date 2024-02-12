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

/// Creates a 401 error response
pub(crate) fn create_unauthorized_response() -> Response {
    (StatusCode::UNAUTHORIZED, Json(json!(
        {
            "http_response": {
                "code": 401,
                "message": "You are unauthorized to access the requested resource."
            },
            "code": 18,
            "description": "",
            "details": {},
            "message": "No SEC header present in request. Please provide it via \"SEC: token\". You may also use BASIC authentication parameters if this host supports it. e.g. \"Authorization: Basic base64Encoding\""
        }
    ))).into_response()
}
