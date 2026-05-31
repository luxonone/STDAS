use loco_rs::testing::prelude::*;
use serde_json::Value;
use stdas_gateway::app::App;

#[tokio::test]
async fn health_endpoint_returns_success_envelope() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/v1/system/health").await;

        assert_eq!(response.status_code(), 200);

        let payload = response.json::<Value>();
        assert_eq!(payload["code"], 0);
        assert_eq!(payload["message"], "success");
        assert_eq!(payload["data"]["service"], "stdas-gateway");
        assert_eq!(payload["data"]["status"], "ok");
    })
    .await;
}

#[tokio::test]
async fn preflight_endpoint_returns_contract_data() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/v1/system/preflight").await;

        assert_eq!(response.status_code(), 200);

        let payload = response.json::<Value>();
        assert_eq!(payload["code"], 0);
        assert_eq!(payload["message"], "success");
        assert_eq!(payload["data"]["gateway"], "stdas-gateway");
        assert_eq!(payload["data"]["api_prefix"], "/api/v1");
        assert_eq!(payload["data"]["purpose"], "phase-0-minimal-verification");
    })
    .await;
}
