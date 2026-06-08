use sqlx::{postgres::PgPoolOptions, PgPool};

/// Create the shared PostgreSQL pool used by module repositories.
///
/// # Errors
///
/// Returns an error if SQLx cannot connect to the configured database.
pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

/// Run SQLx migrations packaged with `stdas-gateway`.
///
/// # Errors
///
/// Returns an error if a migration cannot be applied.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
