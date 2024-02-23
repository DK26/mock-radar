use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;

/// Creates a 422 error response for missing a query parameter
pub(crate) fn create_unprocessable_entity_query_parameter_missing_response(
    missing_field_name: &str,
) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(
        {
            "http_response": {
                "code": 422,
                "message": "The request was well-formed but was unable to be followed due to semantic errors"
            },
            "code": 8,
            "description": "",
            "details": {},
            "message": format!("Missing required parameter \"{missing_field_name}\" from query parameters")
        }
    ))).into_response()
}

/// Creates a 422 error response for query parameter type mismatch
pub(crate) fn create_unprocessable_entity_query_parameter_type_mismatch_response(
    mismatched_field_name: &str,
) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(
        {
            "http_response": {
                "code": 422,
                "message": "The request was well-formed but was unable to be followed due to semantic errors"
            },
            "code": 11,
            "description": "",
            "details": {},
            "message": format!("Failed to transform user query parameter \"{mismatched_field_name}\" with a content type of \"TEXT_PLAIN\"")
        }
    ))).into_response()
}

/// Creates a 422 error response for query parameter `select` having invalid field name
pub(crate) fn create_unprocessable_entity_query_parameter_select_invalid_field_name_response(
    invalid_field_name: &str,
) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(
        {
            "http_response": {
                "code": 422,
                "message": "The request was well-formed but was unable to be followed due to semantic errors"
            },
            "code": 30,
            "description": "",
            "details": {},
            "message": format!("Specified field \"{invalid_field_name}\" is not recognized by this endpoint for media type \"application/json\"")
        }
    ))).into_response()
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

pub(crate) fn create_forbidden_response() -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!(
            {
                "http_response": {
                    "code": 403,
                    "message": "Your account is not authorized to access the requested resource"
                },
                "code": 26,
                "description": "",
                "details": {},
                "message": "User has insufficient capabilities to access this endpoint resource"
            }
        )),
    )
        .into_response()
}
