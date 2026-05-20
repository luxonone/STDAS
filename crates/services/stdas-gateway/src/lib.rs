use axum::{routing::get, Json, Router};
use serde::Serialize;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub code: i32,
    pub message: &'static str,
    pub data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success",
            data,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HealthData {
    pub service: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct PreflightData {
    pub gateway: &'static str,
    pub api_prefix: &'static str,
    pub purpose: &'static str,
}

pub fn app() -> Router {
    Router::new()
        .route("/api/v1/system/health", get(system_health))
        .route("/api/v1/system/preflight", get(system_preflight))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn system_health() -> Json<ApiResponse<HealthData>> {
    Json(ApiResponse::success(HealthData {
        service: "stdas-gateway",
        status: "ok",
    }))
}

async fn system_preflight() -> Json<ApiResponse<PreflightData>> {
    Json(ApiResponse::success(PreflightData {
        gateway: "stdas-gateway",
        api_prefix: "/api/v1",
        purpose: "phase-0-minimal-verification",
    }))
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_endpoint_returns_success_envelope() -> Result<(), Box<dyn std::error::Error>> {
        let request = Request::builder()
            .uri("/api/v1/system/health")
            .body(Body::empty())?;

        let response = app().oneshot(request).await?;

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await?.to_bytes();
        let payload: Value = serde_json::from_slice(&body)?;

        assert_eq!(payload["code"], 0);
        assert_eq!(payload["message"], "success");
        assert_eq!(payload["data"]["service"], "stdas-gateway");
        assert_eq!(payload["data"]["status"], "ok");

        Ok(())
    }
}
