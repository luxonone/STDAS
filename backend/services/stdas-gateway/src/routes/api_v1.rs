use axum::Router;

use crate::{
    modules::{data_pipeline, identity},
    state::AppState,
    system,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(system::router())
        .merge(identity::router())
        .merge(data_pipeline::router())
}
