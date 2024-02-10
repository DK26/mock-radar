use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::permissions;

#[derive(Debug)]
pub struct Permissions(pub permissions::AuthorizationToken);

#[axum::async_trait]
impl<S> FromRequestParts<S> for Permissions
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // TODO: Validate required behavior for when both `Authorization` & `SEC` headers are provided, but one of them is wrong.
        let maybe_authentication_token = parts
            .headers
            .get("SEC")
            .map(|sec_header_value| {
                permissions::Authentication::SecToken(
                    sec_header_value
                        .to_str()
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                )
            })
            .or_else(|| {
                parts
                    .headers
                    .get("Authorization")
                    .and_then(|authorization_header_value| {
                        let mut parts = authorization_header_value
                            .to_str()
                            .unwrap_or_default()
                            .split_whitespace();

                        parts
                            .next()
                            .and_then(|header_type| {
                                (header_type.to_lowercase() == "basic").then_some(parts.next().map(
                                    |basic_token| {
                                        permissions::Authentication::BasicToken(
                                            basic_token.to_string(),
                                        )
                                    },
                                ))
                            })
                            .flatten()
                    })
            });

        let authentication_token = maybe_authentication_token.and_then(permissions::AuthorizationToken::validate).ok_or_else(||(StatusCode::UNAUTHORIZED, Json(json!(
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
        ))).into_response())?;

        Ok(Permissions(authentication_token))
    }
}
