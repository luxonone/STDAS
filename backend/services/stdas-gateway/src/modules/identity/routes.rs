use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(super::handlers::login))
        .route("/auth/me", get(super::handlers::me))
}
