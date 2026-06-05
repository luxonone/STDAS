use axum::{
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::shared::{ApiErrorResponse, ApiResponse};

use super::{
    dto::{LoginRequest, LoginResponse, UserResponse},
    models::AuthError,
};

pub(super) async fn login(Json(request): Json<LoginRequest>) -> Response {
    match super::service::login(&request.username, &request.password) {
        Ok(session) => Json(ApiResponse::success(LoginResponse::from(session))).into_response(),
        Err(AuthError::InvalidCredentials | AuthError::InvalidToken) => unauthorized_response(),
    }
}

pub(super) async fn me(headers: HeaderMap) -> Response {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    match super::service::authenticate_bearer(authorization) {
        Ok(user) => Json(ApiResponse::success(UserResponse::from(user))).into_response(),
        Err(AuthError::InvalidCredentials | AuthError::InvalidToken) => unauthorized_response(),
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
