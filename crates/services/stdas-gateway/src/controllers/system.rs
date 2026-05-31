use loco_rs::prelude::*;
use serde::Serialize;

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

#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .add("/api/v1/system/health", get(system_health))
        .add("/api/v1/system/preflight", get(system_preflight))
}

async fn system_health() -> Result<Response> {
    format::json(ApiResponse::success(HealthData {
        service: "stdas-gateway",
        status: "ok",
    }))
}

async fn system_preflight() -> Result<Response> {
    format::json(ApiResponse::success(PreflightData {
        gateway: "stdas-gateway",
        api_prefix: "/api/v1",
        purpose: "phase-0-minimal-verification",
    }))
}

#[cfg(test)]
mod tests {
    use super::routes;
    use axum::{http::StatusCode, response::IntoResponse};
    use http_body_util::BodyExt;
    use serde_json::Value;

    #[test]
    fn routes_expose_preflight_contract_paths() {
        let paths = routes()
            .handlers
            .into_iter()
            .map(|handler| handler.uri)
            .collect::<Vec<_>>();

        assert_eq!(
            paths,
            vec![
                "/api/v1/system/health".to_string(),
                "/api/v1/system/preflight".to_string()
            ]
        );
    }

    #[tokio::test]
    async fn health_endpoint_returns_success_envelope() -> Result<(), Box<dyn std::error::Error>> {
        let response = super::system_health().await?.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await?.to_bytes();
        let payload: Value = serde_json::from_slice(&body)?;

        assert_eq!(payload["code"], 0);
        assert_eq!(payload["message"], "success");
        assert_eq!(payload["data"]["service"], "stdas-gateway");
        assert_eq!(payload["data"]["status"], "ok");

        Ok(())
    }

    #[tokio::test]
    async fn preflight_endpoint_returns_contract_data() -> Result<(), Box<dyn std::error::Error>> {
        let response = super::system_preflight().await?.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await?.to_bytes();
        let payload: Value = serde_json::from_slice(&body)?;

        assert_eq!(payload["code"], 0);
        assert_eq!(payload["message"], "success");
        assert_eq!(payload["data"]["gateway"], "stdas-gateway");
        assert_eq!(payload["data"]["api_prefix"], "/api/v1");
        assert_eq!(payload["data"]["purpose"], "phase-0-minimal-verification");

        Ok(())
    }
}
