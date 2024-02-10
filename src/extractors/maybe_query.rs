use std::convert::Infallible;

use axum::extract::{rejection::QueryRejection, FromRequestParts};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct MaybeQuery<T>(pub Option<T>);

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for MaybeQuery<T>
where
    T: DeserializeOwned,
    axum::extract::Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(
            match axum::extract::Query::<T>::from_request_parts(parts, state).await {
                Ok(value) => Self(Some(value.0)),
                Err(_) => Self(None),
            },
        )
    }
}
