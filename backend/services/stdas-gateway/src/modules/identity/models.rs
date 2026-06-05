#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatedUser {
    pub username: &'static str,
    pub display_name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoginSession {
    pub access_token: &'static str,
    pub token_type: &'static str,
    pub expires_in_seconds: u32,
    pub user: AuthenticatedUser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError {
    InvalidCredentials,
    InvalidToken,
}
