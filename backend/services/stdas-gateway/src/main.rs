use stdas_gateway::{config::AppConfig, server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env();

    match std::env::args().nth(1).as_deref() {
        Some("routes") => {
            server::print_routes();
            Ok(())
        }
        Some("seed-dev-admin") => server::seed_dev_admin(config).await,
        _ => server::serve(config).await,
    }
}
