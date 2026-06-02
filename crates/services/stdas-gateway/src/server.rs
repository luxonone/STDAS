use tokio::net::TcpListener;

use crate::{app, config::AppConfig, routes, state::AppState};

pub fn print_routes() {
    for route in routes::route_specs() {
        println!("{} {}", route.method, route.path);
    }
}

pub async fn serve(config: AppConfig) -> std::io::Result<()> {
    let listener = TcpListener::bind(&config.bind_addr).await?;

    println!("stdas-gateway listening on http://{}", config.bind_addr);

    axum::serve(listener, app::build_app(AppState::default())).await
}
