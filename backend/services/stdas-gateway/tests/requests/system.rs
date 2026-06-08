use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::Value;
use stdas_gateway::{app::build_app, state::AppState};
use tower::ServiceExt;

fn test_state() -> AppState {
    AppState::for_test_user("admin", "admin@123", "STDAS Administrator")
}

#[tokio::test]
async fn health_endpoint_returns_success_envelope() -> Result<(), Box<dyn std::error::Error>> {
    let response = build_app(test_state())
        .oneshot(
            Request::builder()
                .uri("/api/v1/system/health")
                .body(Body::empty())?,
        )
        .await?;

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
    let response = build_app(test_state())
        .oneshot(
            Request::builder()
                .uri("/api/v1/system/preflight")
                .body(Body::empty())?,
        )
        .await?;

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
