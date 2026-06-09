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
        .map_err(std::io::Error::other)?;

    println!("stdas-gateway listening on http://{}", config.bind_addr);

    axum::serve(listener, app::build_app(state)).await
}

pub async fn seed_dev_admin(config: AppConfig) -> std::io::Result<()> {
    seed_dev_admin_inner(config).await.map_err(io::Error::other)
}

async fn seed_dev_admin_inner(config: AppConfig) -> Result<(), SeedDevAdminError> {
    let admin = bootstrap_admin_from_env_or_prompt()?;
    let pool = crate::db::connect(&config.database_url).await?;
    crate::db::run_migrations(&pool).await?;
    let repository = PgIdentityRepository::new(pool);

    repository.upsert_bootstrap_admin(&admin).await?;
    println!("stdas-gateway bootstrap admin is ready");

    Ok(())
}

fn bootstrap_admin_from_env_or_prompt() -> Result<BootstrapAdmin, SeedDevAdminError> {
    match BootstrapAdmin::from_env() {
        Ok(admin) => Ok(admin),
        Err(BootstrapAdminError::MissingPassword) => {
            println!("STDAS_BOOTSTRAP_ADMIN_PASSWORD is not set.");
            let password = prompt_password("Bootstrap admin password: ")?;
            let confirm = prompt_password("Confirm bootstrap admin password: ")?;
            if password != confirm {
                return Err(SeedDevAdminError::PasswordConfirmationMismatch);
            }

            Ok(BootstrapAdmin::from_env_with_password(password)?)
        }
    }
}

fn prompt_password(prompt: &str) -> Result<String, SeedDevAdminError> {
    rpassword::prompt_password(prompt).map_err(SeedDevAdminError::PasswordPrompt)
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
    #[error("bootstrap admin password input failed")]
    PasswordPrompt(#[source] io::Error),
    #[error("bootstrap admin password confirmation did not match")]
    PasswordConfirmationMismatch,
}
