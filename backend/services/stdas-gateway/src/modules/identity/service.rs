use super::models::{AuthError, AuthenticatedUser, LoginSession};

const ADMIN_USERNAME: &str = "admin";
const ADMIN_PASSWORD: &str = "admin@123";
const ADMIN_ACCESS_TOKEN: &str = "stdas-dev-admin-token";
const TOKEN_TYPE: &str = "Bearer";
const TOKEN_EXPIRES_IN_SECONDS: u32 = 8_u32 * 60_u32 * 60_u32;

pub fn login(username: &str, password: &str) -> Result<LoginSession, AuthError> {
    if username.trim() == ADMIN_USERNAME && password == ADMIN_PASSWORD {
        Ok(LoginSession {
            access_token: ADMIN_ACCESS_TOKEN,
            token_type: TOKEN_TYPE,
            expires_in_seconds: TOKEN_EXPIRES_IN_SECONDS,
            user: admin_user(),
        })
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

pub fn authenticate_bearer(value: Option<&str>) -> Result<AuthenticatedUser, AuthError> {
    let Some(value) = value else {
        return Err(AuthError::InvalidToken);
    };

    let Some(token) = value.strip_prefix("Bearer ") else {
        return Err(AuthError::InvalidToken);
    };

    if token == ADMIN_ACCESS_TOKEN {
        Ok(admin_user())
    } else {
        Err(AuthError::InvalidToken)
    }
}

fn admin_user() -> AuthenticatedUser {
    AuthenticatedUser {
        username: ADMIN_USERNAME,
        display_name: "STDAS Administrator",
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn login_accepts_initial_admin_credentials() {
        let session = super::login("admin", "admin@123").expect("admin login must succeed");

        assert_eq!(session.access_token, "stdas-dev-admin-token");
        assert_eq!(session.user.username, "admin");
        assert_eq!(session.user.display_name, "STDAS Administrator");
    }

    #[test]
    fn login_rejects_invalid_password() {
        assert_eq!(
            super::login("admin", "admin").err(),
            Some(super::AuthError::InvalidCredentials)
        );
    }
}
