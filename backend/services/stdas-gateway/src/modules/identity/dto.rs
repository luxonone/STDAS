use serde::{Deserialize, Serialize};

use super::models::{AuthenticatedUser, LoginSession};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    user_id: String,
    username: String,
    display_name: String,
    person_code: String,
    site_id: String,
    is_system_manager: bool,
}

impl From<AuthenticatedUser> for UserResponse {
    fn from(user: AuthenticatedUser) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            display_name: user.display_name,
            person_code: user.person_code,
            site_id: user.site_id,
            is_system_manager: user.is_system_manager,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    access_token: String,
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
