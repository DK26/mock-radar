mod handlers;
mod permissions;
mod qradar;

use std::sync::{Arc, RwLock};

use axum::Router;

use crate::handlers::general;
use crate::handlers::reference_data;

pub use permissions::REGISTERED_BASIC;
pub use permissions::REGISTERED_PASSWORD;
pub use permissions::REGISTERED_TOKEN;
pub use permissions::REGISTERED_USERNAME;

use crate::qradar::qradar_mock::QRadarMock;

pub type SharedQRadarMock = Arc<RwLock<QRadarMock>>;

pub fn create_routes() -> Router<SharedQRadarMock> {
    Router::new()
        .route("/", axum::routing::get(general::root))
        .merge(reference_data::create_routes())
}
