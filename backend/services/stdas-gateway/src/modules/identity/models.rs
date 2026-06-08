#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub person_code: String,
    pub site_id: String,
    pub is_system_manager: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginSession {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in_seconds: u32,
    pub user: AuthenticatedUser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError {
    InvalidCredentials,
    InvalidToken,
}
