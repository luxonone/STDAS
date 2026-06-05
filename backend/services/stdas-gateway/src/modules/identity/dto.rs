use serde::{Deserialize, Serialize};

use super::models::{AuthenticatedUser, LoginSession};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct UserResponse {
    username: &'static str,
    display_name: &'static str,
}

impl From<AuthenticatedUser> for UserResponse {
    fn from(user: AuthenticatedUser) -> Self {
        Self {
            username: user.username,
            display_name: user.display_name,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LoginResponse {
    access_token: &'static str,
    token_type: &'static str,
    expires_in_seconds: u32,
    user: UserResponse,
}

impl From<LoginSession> for LoginResponse {
    fn from(session: LoginSession) -> Self {
        Self {
            access_token: session.access_token,
            token_type: session.token_type,
            expires_in_seconds: session.expires_in_seconds,
            user: UserResponse::from(session.user),
        }
    }
}
