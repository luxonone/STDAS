use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    dto::{ApiResponse, HealthResponse, PreflightResponse},
    state::AppState,
};

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let status = state.system_service().health();
    Json(ApiResponse::success(HealthResponse::from(status)))
}

pub async fn preflight(State(state): State<AppState>) -> impl IntoResponse {
    let preflight = state.system_service().preflight();
    Json(ApiResponse::success(PreflightResponse::from(preflight)))
}

#[cfg(test)]
mod tests {
    use axum::{body::to_bytes, http::StatusCode, response::IntoResponse};
    use serde_json::Value;

    use crate::state::AppState;

    #[tokio::test]
    async fn health_endpoint_returns_success_envelope() -> Result<(), Box<dyn std::error::Error>> {
        let response = super::health(axum::extract::State(AppState::default()))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 * 1024).await?;
        let payload: Value = serde_json::from_slice(&body)?;

        assert_eq!(payload["code"], 0_i32);
        assert_eq!(payload["message"], "success");
        assert_eq!(payload["data"]["service"], "stdas-gateway");
        assert_eq!(payload["data"]["status"], "ok");

        Ok(())
    }

    #[tokio::test]
    async fn preflight_endpoint_returns_contract_data() -> Result<(), Box<dyn std::error::Error>> {
        let response = super::preflight(axum::extract::State(AppState::default()))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 * 1024).await?;
        let payload: Value = serde_json::from_slice(&body)?;

        assert_eq!(payload["code"], 0_i32);
        assert_eq!(payload["message"], "success");
        assert_eq!(payload["data"]["gateway"], "stdas-gateway");
        assert_eq!(payload["data"]["api_prefix"], "/api/v1");
        assert_eq!(payload["data"]["purpose"], "phase-0-minimal-verification");

        Ok(())
    }
}
