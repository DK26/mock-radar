use axum::Router;

use crate::SharedQRadarMock;

pub(crate) mod sets;

pub(crate) fn create_routes() -> Router<SharedQRadarMock> {
    Router::new().nest(
        "/reference_data",
        Router::new()
            .route("/sets", axum::routing::get(sets::get_handler))
            .route("/sets", axum::routing::post(sets::post_handler)),
    )
}
