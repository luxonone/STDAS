use axum::Router;

use crate::{modules::identity, state::AppState, system};

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(system::router())
        .merge(identity::router())
}
