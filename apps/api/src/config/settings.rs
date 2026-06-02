pub const DEFAULT_BIND_ADDR: &str = "127.0.0.1:8080";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub bind_addr: String,
}

impl AppConfig {
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            bind_addr: std::env::var("STDAS_GATEWAY_ADDR")
                .unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned()),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bind_addr: DEFAULT_BIND_ADDR.to_owned(),
        }
    }
}
