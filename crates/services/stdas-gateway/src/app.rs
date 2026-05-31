use loco_rs::{
    app::{AppContext, Hooks},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    prelude::*,
    task::Tasks,
};
use std::path::Path;

use crate::controllers;

pub struct App;

const SERVICE_CONFIG_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/config");

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::empty().add_route(controllers::system::routes())
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self>(mode, environment, config).await
    }

    async fn load_config(environment: &Environment) -> Result<Config> {
        if std::env::var("LOCO_CONFIG_FOLDER").is_ok() {
            environment.load()
        } else {
            environment.load_from_folder(Path::new(SERVICE_CONFIG_DIR))
        }
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(_tasks: &mut Tasks) {}
}
