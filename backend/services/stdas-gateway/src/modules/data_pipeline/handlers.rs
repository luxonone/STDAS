use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    shared::{ApiErrorResponse, ApiResponse},
    state::AppState,
};

use super::{dto::LotListQuery, service};

pub async fn list_lots(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<LotListQuery>,
) -> Response {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    match state.identity().authenticate_bearer(authorization).await {
        Ok(_) => Json(ApiResponse::success(service::list_lots(&query))).into_response(),
        Err(_) => unauthorized_response(),
    }
}

fn unauthorized_response() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiErrorResponse::new(
            40101_i32,
            "invalid username, password, or token",
        )),
    )
        .into_response()
}
