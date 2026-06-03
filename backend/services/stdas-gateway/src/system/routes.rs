use axum::{routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/system/health",
            get(|| async { super::handlers::health() }),
        )
        .route(
            "/system/preflight",
            get(|| async { super::handlers::preflight() }),
        )
}
