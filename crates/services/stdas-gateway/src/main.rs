use std::{env, net::SocketAddr};

use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let addr = env::var("STDAS_GATEWAY_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
        .parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr).await?;

    tracing::info!(%addr, "stdas gateway listening");
    axum::serve(listener, stdas_gateway::app()).await?;

    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("stdas_gateway=debug,tower_http=debug,info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
