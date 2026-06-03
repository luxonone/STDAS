use axum::Router;

use crate::{state::AppState, system};

pub fn router() -> Router<AppState> {
    Router::new().merge(system::router())
}
