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

async fn login_token(app: axum::Router) -> Result<String, Box<dyn std::error::Error>> {
    let login_response = app
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

    Ok(login_payload["data"]["access_token"]
        .as_str()
        .expect("login response must contain an access token")
        .to_owned())
}

#[tokio::test]
async fn lots_rejects_missing_token() -> Result<(), Box<dyn std::error::Error>> {
    let response = build_app(test_state())
        .oneshot(
            Request::builder()
                .uri("/api/v1/data/lots?cust=AC")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = to_bytes(response.into_body(), 1024 * 1024).await?;
    let payload: Value = serde_json::from_slice(&body)?;
    assert_eq!(payload["code"], 40101_i32);

    Ok(())
}

#[tokio::test]
async fn lots_returns_ft_rows_after_customer_filter() -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app(test_state());
    let access_token = login_token(app.clone()).await?;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/data/lots?cust=AC&page=1&page_size=50")
                .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024 * 1024).await?;
    let payload: Value = serde_json::from_slice(&body)?;
    assert_eq!(payload["code"], 0_i32);
    assert_eq!(payload["data"]["query"]["cust"], "AC");
    assert_eq!(payload["data"]["query"]["test_scope"], "FT (Final Test)");
    assert_eq!(payload["data"]["pagination"]["total"], 5_i32);
    assert!(payload["data"]["items"]
        .as_array()
        .expect("items must be an array")
        .iter()
        .all(|item| item["cust"] == "AC"
            && item["test_scope"] == "FT (Final Test)"
            && item["test_step"] != "CP1"));

    Ok(())
}

#[tokio::test]
async fn lots_default_query_keeps_table_empty() -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app(test_state());
    let access_token = login_token(app.clone()).await?;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/data/lots")
                .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024 * 1024).await?;
    let payload: Value = serde_json::from_slice(&body)?;
    assert_eq!(
        payload["data"]["summary"]["dataset_state"],
        "waiting_for_filters"
    );
    assert_eq!(payload["data"]["pagination"]["total"], 0_i32);
    assert!(payload["data"]["items"]
        .as_array()
        .expect("items must be an array")
        .is_empty());

    Ok(())
}
