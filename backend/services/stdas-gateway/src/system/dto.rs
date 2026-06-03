use serde::Serialize;

use super::models::{HealthStatus, PreflightInfo};

#[derive(Debug, Clone, Copy, Serialize)]
pub(super) struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

impl From<HealthStatus> for HealthResponse {
    fn from(status: HealthStatus) -> Self {
        Self {
            service: status.service,
            status: status.status,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub(super) struct PreflightResponse {
    gateway: &'static str,
    api_prefix: &'static str,
    purpose: &'static str,
}

impl From<PreflightInfo> for PreflightResponse {
    fn from(preflight: PreflightInfo) -> Self {
        Self {
            gateway: preflight.gateway,
            api_prefix: preflight.api_prefix,
            purpose: preflight.purpose,
        }
    }
}
