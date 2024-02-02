use axum::Router;

use crate::SharedQRadarMock;

use super::global;

pub(crate) mod reference_data;

pub(crate) fn create_routes() -> Router<SharedQRadarMock> {
    // TODO: Merge any other handlers under `/api` to the router
    Router::new()
        .nest("/api", reference_data::create_routes())
        .fallback(global::not_found_api_handler)
}
