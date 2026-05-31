#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    loco_rs::cli::main::<stdas_gateway::app::App>().await
}
