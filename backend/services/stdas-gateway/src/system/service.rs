use super::models::{HealthStatus, PreflightInfo};

pub fn health() -> HealthStatus {
    HealthStatus {
        service: "stdas-gateway",
        status: "ok",
    }
}

pub fn preflight() -> PreflightInfo {
    PreflightInfo {
        gateway: "stdas-gateway",
        api_prefix: "/api/v1",
        purpose: "phase-0-minimal-verification",
    }
}
