use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use serde_json::{json, Value};
use stdas_gateway::{app::build_app, state::AppState};
use tower::ServiceExt;

fn test_state() -> AppState {
    AppState::for_test_user("admin", "admin@123", "STDAS Administrator")
}

#[tokio::test]
async fn login_accepts_initial_admin_credentials() -> Result<(), Box<dyn std::error::Error>> {
    let response = build_app(test_state())
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
    assert!(payload["data"]["access_token"]
        .as_str()
        .expect("token must be a string")
        .starts_with("stdas-"));
    assert_eq!(payload["data"]["token_type"], "Bearer");
    assert_eq!(payload["data"]["expires_in_seconds"], 28800_i32);
    assert_eq!(
        payload["data"]["user"]["user_id"],
        "73d29518-9b9d-45c8-a84a-c8df19d9bbd7"
    );
    assert_eq!(payload["data"]["user"]["username"], "admin");
    assert_eq!(
        payload["data"]["user"]["display_name"],
        "STDAS Administrator"
    );
    assert_eq!(payload["data"]["user"]["person_code"], "admin");
    assert_eq!(payload["data"]["user"]["site_id"], "STDAS");
    assert_eq!(payload["data"]["user"]["is_system_manager"], true);

    Ok(())
}

#[tokio::test]
async fn login_rejects_invalid_credentials() -> Result<(), Box<dyn std::error::Error>> {
    let response = build_app(test_state())
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
    let app = build_app(test_state());
    let login_response = app
        .clone()
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
    let login_body = to_bytes(login_response.into_body(), 1024 * 1024).await?;
    let login_payload: Value = serde_json::from_slice(&login_body)?;
    let access_token = login_payload["data"]["access_token"]
        .as_str()
        .expect("login response must contain an access token");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/me")
                .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024 * 1024).await?;
    let payload: Value = serde_json::from_slice(&body)?;
    assert_eq!(payload["code"], 0_i32);
    assert_eq!(
        payload["data"]["user_id"],
        "73d29518-9b9d-45c8-a84a-c8df19d9bbd7"
    );
    assert_eq!(payload["data"]["username"], "admin");
    assert_eq!(payload["data"]["display_name"], "STDAS Administrator");
    assert_eq!(payload["data"]["person_code"], "admin");
    assert_eq!(payload["data"]["site_id"], "STDAS");
    assert_eq!(payload["data"]["is_system_manager"], true);

    Ok(())
}
