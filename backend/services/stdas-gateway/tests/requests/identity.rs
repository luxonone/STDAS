use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use serde_json::{json, Value};
use stdas_gateway::{app::build_app, state::AppState};
use tower::ServiceExt;

#[tokio::test]
async fn login_accepts_initial_admin_credentials() -> Result<(), Box<dyn std::error::Error>> {
    let response = build_app(AppState)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "admin",
                        "password": "admin@123"
                    })
                    .to_string(),
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024 * 1024).await?;
    let payload: Value = serde_json::from_slice(&body)?;
    assert_eq!(payload["code"], 0_i32);
    assert_eq!(payload["message"], "success");
    assert_eq!(payload["data"]["access_token"], "stdas-dev-admin-token");
    assert_eq!(payload["data"]["token_type"], "Bearer");
    assert_eq!(payload["data"]["user"]["username"], "admin");

    Ok(())
}

#[tokio::test]
async fn login_rejects_invalid_credentials() -> Result<(), Box<dyn std::error::Error>> {
    let response = build_app(AppState)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "admin",
                        "password": "admin"
                    })
                    .to_string(),
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = to_bytes(response.into_body(), 1024 * 1024).await?;
    let payload: Value = serde_json::from_slice(&body)?;
    assert_eq!(payload["code"], 40101_i32);
    assert_eq!(payload["data"], Value::Null);

    Ok(())
}

#[tokio::test]
async fn me_returns_current_admin_for_valid_token() -> Result<(), Box<dyn std::error::Error>> {
    let response = build_app(AppState)
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/me")
                .header(header::AUTHORIZATION, "Bearer stdas-dev-admin-token")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024 * 1024).await?;
    let payload: Value = serde_json::from_slice(&body)?;
    assert_eq!(payload["code"], 0_i32);
    assert_eq!(payload["data"]["username"], "admin");
    assert_eq!(payload["data"]["display_name"], "STDAS Administrator");

    Ok(())
}
