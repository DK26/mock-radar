pub(crate) mod sets;

use axum::Router;

use crate::SharedQRadarMock;

pub(crate) fn create_routes() -> Router<SharedQRadarMock> {
    Router::new().nest(
        "/reference_data",
        Router::new()
            .route(
                "/sets",
                axum::routing::get(sets::get::get_reference_data_sets_handler),
            )
            .route(
                "/sets",
                axum::routing::post(sets::post::post_reference_data_sets_handler),
            ),
    )
}
