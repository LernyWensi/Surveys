use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod auth;
mod survey_results;
mod surveys;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(auth::router())
        .merge(surveys::router())
        .merge(survey_results::router())
}
