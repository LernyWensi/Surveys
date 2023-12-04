use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod api;

pub fn create() -> Router<Arc<AppState>> {
    Router::new().nest("/api", api::router())
}
