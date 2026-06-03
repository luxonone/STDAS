use tokio::net::TcpListener;

use crate::{app, config::AppConfig, routes, state::AppState};

pub fn print_routes() {
    for route in routes::route_specs() {
        println!("{} {}", route.method, route.path);
    }
}

/// Run the Axum HTTP server until it is stopped.
///
/// # Errors
///
/// Returns an I/O error if the TCP listener cannot bind or if Axum reports a
/// server-level I/O failure.
pub async fn serve(config: AppConfig) -> std::io::Result<()> {
    let listener = TcpListener::bind(&config.bind_addr).await?;

    println!("stdas-gateway listening on http://{}", config.bind_addr);

    axum::serve(listener, app::build_app(AppState)).await
}
