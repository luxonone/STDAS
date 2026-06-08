use std::sync::Arc;

use thiserror::Error;

use crate::{
    config::AppConfig,
    modules::identity::service::{IdentityService, PgIdentityRepository},
};

#[cfg(debug_assertions)]
use crate::modules::identity::service::{StoredUserRecord, TestIdentityRepository};

#[derive(Clone)]
pub struct AppState {
    identity: IdentityService,
}

impl AppState {
    #[must_use]
    pub(crate) fn new(identity: IdentityService) -> Self {
        Self { identity }
    }

    #[must_use]
    pub(crate) fn identity(&self) -> &IdentityService {
        &self.identity
    }

    /// Build application state for a real runtime process.
    ///
    /// # Errors
    ///
    /// Returns an error if the configured PostgreSQL database cannot be opened
    /// or if SQLx migrations fail.
    pub async fn from_config(config: &AppConfig) -> Result<Self, AppStateError> {
        let pool = crate::db::connect(&config.database_url).await?;
        crate::db::run_migrations(&pool).await?;
        let repository = Arc::new(PgIdentityRepository::new(pool));

        Ok(Self::new(IdentityService::new(repository)))
    }

    #[cfg(debug_assertions)]
    #[must_use]
    pub fn for_test_user(username: &str, password: &str, display_name: &str) -> Self {
        let user = StoredUserRecord {
            id: "73d29518-9b9d-45c8-a84a-c8df19d9bbd7".to_owned(),
            username: username.to_owned(),
            passwd: crate::modules::identity::service::hash_password_for_test(password),
            fname: display_name.to_owned(),
            person_code: username.to_owned(),
            site_id: "STDAS".to_owned(),
            is_system_manager: true,
            is_on_job: true,
        };
        let repository = Arc::new(TestIdentityRepository::new().with_user(user));

        Self::new(IdentityService::new(repository))
    }
}

#[derive(Debug, Error)]
pub enum AppStateError {
    #[error("database connection failed")]
    Database(#[from] sqlx::Error),
    #[error("database migration failed")]
    Migration(#[from] sqlx::migrate::MigrateError),
}
