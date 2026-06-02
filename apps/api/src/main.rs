use stdas_gateway::{config::AppConfig, server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env();

    if std::env::args().nth(1).as_deref() == Some("routes") {
        server::print_routes();
        Ok(())
    } else {
        server::serve(config).await
    }
}
