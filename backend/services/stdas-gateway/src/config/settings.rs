pub const DEFAULT_BIND_ADDR: &str = "127.0.0.1:8080";
pub const DEFAULT_DATABASE_URL: &str = "postgres://stdas:stdas@localhost:5432/stdas";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub bind_addr: String,
    pub database_url: String,
}

impl AppConfig {
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            bind_addr: std::env::var("STDAS_GATEWAY_ADDR")
                .unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned()),
            database_url: std::env::var("STDAS_DATABASE_URL")
                .unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned()),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bind_addr: DEFAULT_BIND_ADDR.to_owned(),
            database_url: DEFAULT_DATABASE_URL.to_owned(),
        }
    }
}
