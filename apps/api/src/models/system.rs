#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HealthStatus {
    pub service: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreflightInfo {
    pub gateway: &'static str,
    pub api_prefix: &'static str,
    pub purpose: &'static str,
}
