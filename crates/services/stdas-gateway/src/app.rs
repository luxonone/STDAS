use axum::Router;

use crate::{middleware, routes, state::AppState};

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", routes::api_v1::router())
        .layer(middleware::cors::layer())
        .with_state(state)
}
