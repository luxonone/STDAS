use std::io;

use tokio::net::TcpListener;

use thiserror::Error;

use crate::{
    app,
    config::AppConfig,
    modules::identity::service::{
        BootstrapAdmin, BootstrapAdminError, IdentityRepositoryError, PgIdentityRepository,
    },
    routes,
    state::AppState,
};

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
    let state = AppState::from_config(&config)
        .await
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;

    println!("stdas-gateway listening on http://{}", config.bind_addr);

    axum::serve(listener, app::build_app(state)).await
}

pub async fn seed_dev_admin(config: AppConfig) -> std::io::Result<()> {
    seed_dev_admin_inner(config)
        .await
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
}

async fn seed_dev_admin_inner(config: AppConfig) -> Result<(), SeedDevAdminError> {
    let admin = BootstrapAdmin::from_env()?;
    let pool = crate::db::connect(&config.database_url).await?;
    crate::db::run_migrations(&pool).await?;
    let repository = PgIdentityRepository::new(pool);

    repository.upsert_bootstrap_admin(&admin).await?;
    println!("stdas-gateway bootstrap admin is ready");

    Ok(())
}

#[derive(Debug, Error)]
enum SeedDevAdminError {
    #[error("bootstrap admin configuration failed")]
    Bootstrap(#[from] BootstrapAdminError),
    #[error("database connection failed")]
    Database(#[from] sqlx::Error),
    #[error("database migration failed")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("identity seed failed")]
    Identity(#[from] IdentityRepositoryError),
}
