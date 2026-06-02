use crate::models::{HealthStatus, PreflightInfo};

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemService;

impl SystemService {
    pub fn health(&self) -> HealthStatus {
        HealthStatus {
            service: "stdas-gateway",
            status: "ok",
        }
    }

    pub fn preflight(&self) -> PreflightInfo {
        PreflightInfo {
            gateway: "stdas-gateway",
            api_prefix: "/api/v1",
            purpose: "phase-0-minimal-verification",
        }
    }
}
