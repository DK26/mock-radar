use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    handlers,
    permissions::{self, AuthenticationToken},
};

fn extract_authentication_token(parts: &mut Parts) -> Option<AuthenticationToken> {
    // TODO: Validate required behavior for when both `Authorization` & `SEC` headers are provided, but one of them is wrong.
    parts
        .headers
        .get("SEC")
        .map(|sec_header_value| {
            permissions::AuthenticationToken::Sec(
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
                                    permissions::AuthenticationToken::Basic(basic_token.to_string())
                                },
                            ))
                        })
                        .flatten()
                })
        })
}

#[derive(Debug)]
pub struct Permissions(pub permissions::Permissions);

#[axum::async_trait]
impl<S> FromRequestParts<S> for Permissions
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // TODO: Validate required behavior for when both `Authorization` & `SEC` headers are provided, but one of them is wrong.
        let maybe_authentication_token = extract_authentication_token(parts);

        let authentication_token = maybe_authentication_token
            .and_then(permissions::Permissions::validate)
            .ok_or_else(handlers::errors::response::create_unauthorized_response)?;

        Ok(Permissions(authentication_token))
    }
}

#[derive(Debug)]
pub struct WritePermission(pub permissions::WritePermission);

#[axum::async_trait]
impl<S> FromRequestParts<S> for WritePermission
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let maybe_authentication_token = extract_authentication_token(parts);

        let permissions = maybe_authentication_token
            .and_then(permissions::Permissions::validate)
            .ok_or_else(handlers::errors::response::create_unauthorized_response)?;

        Ok(WritePermission(permissions.try_into().map_err(|_| {
            handlers::errors::response::create_forbidden_response()
        })?))
    }
}

#[derive(Debug)]
pub struct ReadPermission(pub permissions::ReadPermission);

#[axum::async_trait]
impl<S> FromRequestParts<S> for ReadPermission
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // TODO: Validate required behavior for when both `Authorization` & `SEC` headers are provided, but one of them is wrong.
        let maybe_authentication_token = extract_authentication_token(parts);

        let permissions = maybe_authentication_token
            .and_then(permissions::Permissions::validate)
            .ok_or_else(handlers::errors::response::create_unauthorized_response)?;

        Ok(ReadPermission(permissions.try_into().map_err(|_| {
            handlers::errors::response::create_forbidden_response()
        })?))
    }
}
