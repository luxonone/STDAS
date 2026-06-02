use axum::{routing::get, Router};

use crate::{handlers, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/system/health", get(handlers::system::health))
        .route("/system/preflight", get(handlers::system::preflight))
}
