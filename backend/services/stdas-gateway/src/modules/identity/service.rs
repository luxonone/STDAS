use std::{env, fs, io, sync::Arc};

#[cfg(debug_assertions)]
use std::sync::Mutex;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use rand_core::OsRng;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use super::models::{AuthError, AuthenticatedUser, LoginSession};

pub const TOKEN_TYPE: &str = "Bearer";
pub const TOKEN_EXPIRES_IN_SECONDS: u32 = 8_u32 * 60_u32 * 60_u32;
pub const BOOTSTRAP_ADMIN_USERNAME_ENV: &str = "STDAS_BOOTSTRAP_ADMIN_USERNAME";
pub const BOOTSTRAP_ADMIN_PASSWORD_ENV: &str = "STDAS_BOOTSTRAP_ADMIN_PASSWORD";
pub const BOOTSTRAP_ADMIN_PASSWORD_FILE_ENV: &str = "STDAS_BOOTSTRAP_ADMIN_PASSWORD_FILE";
pub const BOOTSTRAP_ADMIN_DISPLAY_NAME_ENV: &str = "STDAS_BOOTSTRAP_ADMIN_DISPLAY_NAME";
pub const BOOTSTRAP_ADMIN_PERSON_CODE_ENV: &str = "STDAS_BOOTSTRAP_ADMIN_PERSON_CODE";
pub const BOOTSTRAP_ADMIN_SITE_ID_ENV: &str = "STDAS_BOOTSTRAP_ADMIN_SITE_ID";

const DEFAULT_BOOTSTRAP_ADMIN_USERNAME: &str = "admin";
const DEFAULT_BOOTSTRAP_ADMIN_DISPLAY_NAME: &str = "STDAS Administrator";
const DEFAULT_BOOTSTRAP_ADMIN_SITE_ID: &str = "STDAS";
const DEFAULT_BOOTSTRAP_ADMIN_PASSWORD_FILES: &[&str] = &[
    "backend/services/stdas-gateway/.local/bootstrap-admin-password",
    ".local/bootstrap-admin-password",
];

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub(crate) struct StoredUserRecord {
    pub id: String,
    pub username: String,
    pub passwd: String,
    pub fname: String,
    pub person_code: String,
    pub site_id: String,
    pub is_system_manager: bool,
    pub is_on_job: bool,
}

impl StoredUserRecord {
    fn into_authenticated_user(self) -> AuthenticatedUser {
        let display_name = if self.fname.trim().is_empty() {
            self.username.clone()
        } else {
            self.fname
        };

        AuthenticatedUser {
            user_id: self.id,
            username: self.username,
            display_name,
            person_code: self.person_code,
            site_id: self.site_id,
            is_system_manager: self.is_system_manager,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BootstrapAdmin {
    username: String,
    password: String,
    display_name: String,
    person_code: String,
    site_id: String,
}

impl BootstrapAdmin {
    pub(crate) fn from_env() -> Result<Self, BootstrapAdminError> {
        Self::from_sources(|key| env::var(key).ok(), read_bootstrap_password_file)
    }

    fn from_sources(
        mut lookup: impl FnMut(&str) -> Option<String>,
        mut read_password_file: impl FnMut(&str) -> Result<Option<String>, BootstrapAdminError>,
    ) -> Result<Self, BootstrapAdminError> {
        let username = lookup(BOOTSTRAP_ADMIN_USERNAME_ENV)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| DEFAULT_BOOTSTRAP_ADMIN_USERNAME.to_owned());
        let configured_password_file = lookup(BOOTSTRAP_ADMIN_PASSWORD_FILE_ENV)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let password = match configured_password_file {
            Some(path) => Some(
                read_password_file(&path)?
                    .ok_or_else(|| BootstrapAdminError::PasswordFileMissing { path })?,
            ),
            None => read_default_bootstrap_password_file(&mut read_password_file)?.or_else(|| {
                lookup(BOOTSTRAP_ADMIN_PASSWORD_ENV).filter(|value| !value.trim().is_empty())
            }),
        }
        .ok_or(BootstrapAdminError::MissingPassword)?;
        let display_name = lookup(BOOTSTRAP_ADMIN_DISPLAY_NAME_ENV)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| DEFAULT_BOOTSTRAP_ADMIN_DISPLAY_NAME.to_owned());
        let person_code = lookup(BOOTSTRAP_ADMIN_PERSON_CODE_ENV)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| username.clone());
        let site_id = lookup(BOOTSTRAP_ADMIN_SITE_ID_ENV)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| DEFAULT_BOOTSTRAP_ADMIN_SITE_ID.to_owned());

        Ok(Self {
            username,
            password,
            display_name,
            person_code,
            site_id,
        })
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub(crate) enum BootstrapAdminError {
    #[error("STDAS_BOOTSTRAP_ADMIN_PASSWORD, STDAS_BOOTSTRAP_ADMIN_PASSWORD_FILE, or backend/services/stdas-gateway/.local/bootstrap-admin-password is required to seed the local admin user")]
    MissingPassword,
    #[error("configured bootstrap admin password file was not found: {path}")]
    PasswordFileMissing { path: String },
    #[error("bootstrap admin password file could not be read: {path}")]
    PasswordFileRead { path: String },
}

fn read_default_bootstrap_password_file(
    read_password_file: &mut impl FnMut(&str) -> Result<Option<String>, BootstrapAdminError>,
) -> Result<Option<String>, BootstrapAdminError> {
    for path in DEFAULT_BOOTSTRAP_ADMIN_PASSWORD_FILES {
        if let Some(password) = read_password_file(path)? {
            return Ok(Some(password));
        }
    }

    Ok(None)
}

fn read_bootstrap_password_file(path: &str) -> Result<Option<String>, BootstrapAdminError> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(normalize_password_file_contents(&contents)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(BootstrapAdminError::PasswordFileRead {
            path: path.to_owned(),
        }),
    }
}

fn normalize_password_file_contents(contents: &str) -> Option<String> {
    let password = contents.trim_end_matches(['\r', '\n']).to_owned();
    (!password.is_empty()).then_some(password)
}

#[async_trait]
pub(crate) trait IdentityRepository: Send + Sync {
    async fn find_active_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<StoredUserRecord>, IdentityRepositoryError>;

    async fn create_session(
        &self,
        user_id: &str,
        access_token_hash: &str,
        expires_in_seconds: u32,
    ) -> Result<(), IdentityRepositoryError>;

    async fn find_active_user_by_token_hash(
        &self,
        access_token_hash: &str,
    ) -> Result<Option<StoredUserRecord>, IdentityRepositoryError>;
}

#[derive(Debug, Error)]
pub(crate) enum IdentityRepositoryError {
    #[error("identity database query failed")]
    Database(#[from] sqlx::Error),
    #[error("identity password hashing failed")]
    PasswordHash,
    #[cfg(debug_assertions)]
    #[error("identity test repository lock failed")]
    LockPoisoned,
}

#[derive(Clone)]
pub(crate) struct IdentityService {
    repository: Arc<dyn IdentityRepository>,
}

impl IdentityService {
    #[must_use]
    pub(crate) fn new(repository: Arc<dyn IdentityRepository>) -> Self {
        Self { repository }
    }

    pub(crate) async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginSession, AuthError> {
        let username = username.trim();
        if username.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }

        let user = self
            .repository
            .find_active_user_by_username(username)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?
            .ok_or(AuthError::InvalidCredentials)?;

        if !user.is_on_job || !verify_password(&user.passwd, password) {
            return Err(AuthError::InvalidCredentials);
        }

        let access_token = new_access_token();
        let access_token_hash = hash_access_token(&access_token);
        self.repository
            .create_session(&user.id, &access_token_hash, TOKEN_EXPIRES_IN_SECONDS)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;

        Ok(LoginSession {
            access_token,
            token_type: TOKEN_TYPE,
            expires_in_seconds: TOKEN_EXPIRES_IN_SECONDS,
            user: user.into_authenticated_user(),
        })
    }

    pub(crate) async fn authenticate_bearer(
        &self,
        value: Option<&str>,
    ) -> Result<AuthenticatedUser, AuthError> {
        let Some(value) = value else {
            return Err(AuthError::InvalidToken);
        };

        let Some(token) = value.strip_prefix("Bearer ") else {
            return Err(AuthError::InvalidToken);
        };

        let user = self
            .repository
            .find_active_user_by_token_hash(&hash_access_token(token))
            .await
            .map_err(|_| AuthError::InvalidToken)?
            .ok_or(AuthError::InvalidToken)?;

        Ok(user.into_authenticated_user())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PgIdentityRepository {
    pool: PgPool,
}

impl PgIdentityRepository {
    #[must_use]
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn upsert_bootstrap_admin(
        &self,
        admin: &BootstrapAdmin,
    ) -> Result<(), IdentityRepositoryError> {
        let passwd =
            hash_password(&admin.password).map_err(|_| IdentityRepositoryError::PasswordHash)?;
        let mut transaction = self.pool.begin().await?;

        let user_id = sqlx::query_scalar::<_, String>(
            r#"
            INSERT INTO c_users (
                id,
                username,
                passwd,
                fname,
                site_id,
                department,
                is_system_reserved,
                is_system_manager,
                person_code,
                creat_user,
                lm_user,
                new_account_source,
                is_on_job
            )
            VALUES ($1, $2, $3, $4, $5, 'STDAS', 'Y', 'Y', $6, 'bootstrap', 'bootstrap', 'STDAS', 'Y')
            ON CONFLICT (username)
            DO UPDATE SET
                passwd = EXCLUDED.passwd,
                fname = EXCLUDED.fname,
                site_id = EXCLUDED.site_id,
                department = COALESCE(NULLIF(c_users.department, ''), EXCLUDED.department),
                is_system_manager = 'Y',
                person_code = EXCLUDED.person_code,
                lm_user = 'bootstrap',
                lm_date = now(),
                new_account_source = 'STDAS',
                is_on_job = 'Y',
                depart_date = NULL
            RETURNING id
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&admin.username)
        .bind(passwd)
        .bind(&admin.display_name)
        .bind(&admin.site_id)
        .bind(&admin.person_code)
        .fetch_one(&mut *transaction)
        .await?;

        let role_id = sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO c_roles (
                id,
                site_id,
                role_name,
                is_system_reserved,
                authorized_level,
                roles_uuid,
                decentralization,
                create_user,
                lm_user
            )
            VALUES (1, $1, 'STDAS_ADMIN', 'Y', '9', 'STDAS_ADMIN', 'N', 'bootstrap', 'bootstrap')
            ON CONFLICT (site_id, role_name)
            DO UPDATE SET
                authorized_level = EXCLUDED.authorized_level,
                is_system_reserved = 'Y',
                lm_user = 'bootstrap',
                lm_time = now()
            RETURNING id
            "#,
        )
        .bind(&admin.site_id)
        .fetch_one(&mut *transaction)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO c_user_rl (
                role_id,
                is_system_reserved,
                user_id,
                creat_user,
                creat_date,
                lm_user,
                lm_date
            )
            VALUES ($1, 'Y', $2, 'bootstrap', now(), 'bootstrap', now())
            ON CONFLICT (user_id, role_id)
            DO UPDATE SET
                is_system_reserved = 'Y',
                lm_user = 'bootstrap',
                lm_date = now()
            "#,
        )
        .bind(role_id)
        .bind(user_id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl IdentityRepository for PgIdentityRepository {
    async fn find_active_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<StoredUserRecord>, IdentityRepositoryError> {
        let user = sqlx::query_as::<_, StoredUserRecord>(
            r#"
            SELECT
                id,
                username,
                passwd,
                COALESCE(NULLIF(fname, ''), username) AS fname,
                person_code,
                COALESCE(site_id, '') AS site_id,
                is_system_manager = 'Y' AS is_system_manager,
                is_on_job = 'Y' AS is_on_job
            FROM c_users
            WHERE username = $1
              AND is_on_job = 'Y'
            LIMIT 1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create_session(
        &self,
        user_id: &str,
        access_token_hash: &str,
        expires_in_seconds: u32,
    ) -> Result<(), IdentityRepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO r_user_session (
                session_uuid,
                user_id,
                access_token_hash,
                token_type,
                expires_at,
                is_revoked,
                create_time
            )
            VALUES ($1, $2, $3, $4, now() + ($5::int * interval '1 second'), 'N', now())
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(user_id)
        .bind(access_token_hash)
        .bind(TOKEN_TYPE)
        .bind(i32::try_from(expires_in_seconds).unwrap_or(i32::MAX))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_active_user_by_token_hash(
        &self,
        access_token_hash: &str,
    ) -> Result<Option<StoredUserRecord>, IdentityRepositoryError> {
        let user = sqlx::query_as::<_, StoredUserRecord>(
            r#"
            SELECT
                u.id,
                u.username,
                u.passwd,
                COALESCE(NULLIF(u.fname, ''), u.username) AS fname,
                u.person_code,
                COALESCE(u.site_id, '') AS site_id,
                u.is_system_manager = 'Y' AS is_system_manager,
                u.is_on_job = 'Y' AS is_on_job
            FROM r_user_session s
            JOIN c_users u ON u.id = s.user_id
            WHERE s.access_token_hash = $1
              AND s.token_type = $2
              AND s.is_revoked = 'N'
              AND s.expires_at > now()
              AND u.is_on_job = 'Y'
            LIMIT 1
            "#,
        )
        .bind(access_token_hash)
        .bind(TOKEN_TYPE)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Default)]
pub(crate) struct TestIdentityRepository {
    users: Mutex<Vec<StoredUserRecord>>,
    sessions: Mutex<Vec<TestSessionRecord>>,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestSessionRecord {
    user_id: String,
    access_token_hash: String,
}

#[cfg(debug_assertions)]
impl TestIdentityRepository {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub(crate) fn with_user(self, user: StoredUserRecord) -> Self {
        self.users
            .lock()
            .expect("test identity users lock must not be poisoned")
            .push(user);
        self
    }

    #[cfg(test)]
    pub(crate) async fn session_count(&self) -> usize {
        self.sessions
            .lock()
            .expect("test identity sessions lock must not be poisoned")
            .len()
    }
}

#[cfg(debug_assertions)]
#[async_trait]
impl IdentityRepository for TestIdentityRepository {
    async fn find_active_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<StoredUserRecord>, IdentityRepositoryError> {
        let users = self
            .users
            .lock()
            .map_err(|_| IdentityRepositoryError::LockPoisoned)?;
        Ok(users
            .iter()
            .find(|user| user.username == username && user.is_on_job)
            .cloned())
    }

    async fn create_session(
        &self,
        user_id: &str,
        access_token_hash: &str,
        _expires_in_seconds: u32,
    ) -> Result<(), IdentityRepositoryError> {
        self.sessions
            .lock()
            .map_err(|_| IdentityRepositoryError::LockPoisoned)?
            .push(TestSessionRecord {
                user_id: user_id.to_owned(),
                access_token_hash: access_token_hash.to_owned(),
            });
        Ok(())
    }

    async fn find_active_user_by_token_hash(
        &self,
        access_token_hash: &str,
    ) -> Result<Option<StoredUserRecord>, IdentityRepositoryError> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|_| IdentityRepositoryError::LockPoisoned)?;
        let Some(session) = sessions
            .iter()
            .find(|session| session.access_token_hash == access_token_hash)
        else {
            return Ok(None);
        };

        let users = self
            .users
            .lock()
            .map_err(|_| IdentityRepositoryError::LockPoisoned)?;
        Ok(users
            .iter()
            .find(|user| user.id == session.user_id && user.is_on_job)
            .cloned())
    }
}

fn verify_password(passwd: &str, password: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(passwd) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn new_access_token() -> String {
    format!("stdas-{}", Uuid::new_v4())
}

fn hash_access_token(access_token: &str) -> String {
    let digest = Sha256::digest(access_token.as_bytes());
    hex::encode(digest)
}

#[cfg(debug_assertions)]
pub(crate) fn hash_password_for_test(password: &str) -> String {
    let salt = SaltString::from_b64("c3RkYXMtdGVzdC1zYWx0").expect("test salt is valid");
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("test password hash must be generated")
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{
        BootstrapAdmin, BootstrapAdminError, IdentityService, StoredUserRecord,
        TestIdentityRepository, TOKEN_EXPIRES_IN_SECONDS,
    };

    fn admin_record() -> StoredUserRecord {
        StoredUserRecord {
            id: "73d29518-9b9d-45c8-a84a-c8df19d9bbd7".to_owned(),
            username: "admin".to_owned(),
            passwd: super::hash_password_for_test("admin@123"),
            fname: "STDAS Administrator".to_owned(),
            person_code: "admin".to_owned(),
            site_id: "STDAS".to_owned(),
            is_system_manager: true,
            is_on_job: true,
        }
    }

    #[tokio::test]
    async fn login_reads_active_user_from_repository_and_creates_session() {
        let repository = Arc::new(TestIdentityRepository::new().with_user(admin_record()));
        let service = IdentityService::new(repository.clone());

        let session = service
            .login("admin", "admin@123")
            .await
            .expect("admin login must succeed");

        assert!(session.access_token.starts_with("stdas-"));
        assert_eq!(session.token_type, "Bearer");
        assert_eq!(session.expires_in_seconds, TOKEN_EXPIRES_IN_SECONDS);
        assert_eq!(session.user.username, "admin");
        assert_eq!(session.user.display_name, "STDAS Administrator");
        assert_eq!(repository.session_count().await, 1);
    }

    #[tokio::test]
    async fn login_rejects_departed_user_even_when_password_matches() {
        let mut user = admin_record();
        user.is_on_job = false;
        let repository = Arc::new(TestIdentityRepository::new().with_user(user));
        let service = IdentityService::new(repository.clone());

        let result = service.login("admin", "admin@123").await;

        assert_eq!(result.err(), Some(super::AuthError::InvalidCredentials));
        assert_eq!(repository.session_count().await, 0);
    }

    #[tokio::test]
    async fn authenticate_bearer_reads_current_session_from_repository() {
        let repository = Arc::new(TestIdentityRepository::new().with_user(admin_record()));
        let service = IdentityService::new(repository.clone());
        let session = service
            .login("admin", "admin@123")
            .await
            .expect("admin login must succeed");

        let authorization = format!("Bearer {}", session.access_token);
        let user = service
            .authenticate_bearer(Some(&authorization))
            .await
            .expect("created token must authenticate");

        assert_eq!(user.username, "admin");
        assert_eq!(user.display_name, "STDAS Administrator");
    }

    #[test]
    fn bootstrap_admin_uses_mes_aligned_defaults_and_requires_password() {
        let admin = BootstrapAdmin::from_sources(
            |key| match key {
                super::BOOTSTRAP_ADMIN_PASSWORD_ENV => Some("admin@123".to_owned()),
                _ => None,
            },
            |_| Ok(None),
        )
        .expect("password env should create bootstrap admin");

        assert_eq!(admin.username, "admin");
        assert_eq!(admin.display_name, "STDAS Administrator");
        assert_eq!(admin.person_code, "admin");
        assert_eq!(admin.site_id, "STDAS");

        let missing = BootstrapAdmin::from_sources(|_| None, |_| Ok(None));
        assert_eq!(missing.err(), Some(BootstrapAdminError::MissingPassword));
    }

    #[test]
    fn bootstrap_admin_trims_non_secret_mes_fields() {
        let admin = BootstrapAdmin::from_sources(
            |key| match key {
                super::BOOTSTRAP_ADMIN_USERNAME_ENV => Some(" te_admin ".to_owned()),
                super::BOOTSTRAP_ADMIN_PASSWORD_ENV => Some(" admin@123 ".to_owned()),
                super::BOOTSTRAP_ADMIN_DISPLAY_NAME_ENV => Some(" Test Engineer Admin ".to_owned()),
                super::BOOTSTRAP_ADMIN_PERSON_CODE_ENV => Some(" UW00133 ".to_owned()),
                super::BOOTSTRAP_ADMIN_SITE_ID_ENV => Some(" KYBER ".to_owned()),
                _ => None,
            },
            |_| Ok(None),
        )
        .expect("explicit env values should create bootstrap admin");

        assert_eq!(admin.username, "te_admin");
        assert_eq!(admin.password, " admin@123 ");
        assert_eq!(admin.display_name, "Test Engineer Admin");
        assert_eq!(admin.person_code, "UW00133");
        assert_eq!(admin.site_id, "KYBER");
    }

    #[test]
    fn bootstrap_admin_prefers_local_password_file_for_dev_seed() {
        let admin = BootstrapAdmin::from_sources(
            |key| match key {
                super::BOOTSTRAP_ADMIN_PASSWORD_ENV => Some("env-password".to_owned()),
                _ => None,
            },
            |path| {
                if path == "backend/services/stdas-gateway/.local/bootstrap-admin-password" {
                    Ok(Some("file-password".to_owned()))
                } else {
                    Ok(None)
                }
            },
        )
        .expect("local password file should create bootstrap admin");

        assert_eq!(admin.username, "admin");
        assert_eq!(admin.password, "file-password");
    }

    #[test]
    fn bootstrap_admin_reads_explicit_password_file_without_trailing_newline() {
        let admin = BootstrapAdmin::from_sources(
            |key| match key {
                super::BOOTSTRAP_ADMIN_PASSWORD_FILE_ENV => {
                    Some("D:/local/stdas-admin-password".to_owned())
                }
                _ => None,
            },
            |path| {
                if path == "D:/local/stdas-admin-password" {
                    Ok(Some(
                        super::normalize_password_file_contents("admin@123\r\n")
                            .expect("password should be present"),
                    ))
                } else {
                    Ok(None)
                }
            },
        )
        .expect("explicit local password file should create bootstrap admin");

        assert_eq!(admin.password, "admin@123");
    }

    #[test]
    fn bootstrap_admin_requires_explicit_password_file_to_exist() {
        let missing = BootstrapAdmin::from_sources(
            |key| match key {
                super::BOOTSTRAP_ADMIN_PASSWORD_FILE_ENV => {
                    Some("D:/local/missing-password".to_owned())
                }
                _ => None,
            },
            |_| Ok(None),
        );

        assert_eq!(
            missing.err(),
            Some(BootstrapAdminError::PasswordFileMissing {
                path: "D:/local/missing-password".to_owned()
            })
        );
    }
}
