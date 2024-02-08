mod extractors;
mod handlers;
mod permissions;
mod qradar;

use std::sync::{Arc, RwLock};

use axum::Router;
use handlers::api;

use crate::handlers::global;

pub use permissions::REGISTERED_BASIC_TOKEN;
pub use permissions::REGISTERED_EXPIRED_SEC_TOKEN;
pub use permissions::REGISTERED_PASSWORD;
pub use permissions::REGISTERED_READONLY_SEC_TOKEN;
pub use permissions::REGISTERED_SEC_TOKEN;
pub use permissions::REGISTERED_USERNAME;

use crate::qradar::qradar_mock::QRadarMock;

pub type SharedQRadarMock = Arc<RwLock<QRadarMock>>;

pub fn create_routes() -> Router<SharedQRadarMock> {
    Router::new()
        .route("/", axum::routing::get(global::root))
        .merge(api::create_routes())
        .fallback(global::global_not_found_handler)
}
