//! Stable JSON shapes for API responses (avoid untyped `serde_json::json!` in handlers).

use serde::Serialize;

/// Authenticated user returned to clients (no password material).
#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

/// Successful login, registration, or refresh.
#[derive(Debug, Serialize)]
pub struct AuthSuccess {
    /// Short-lived access JWT (same semantics as `OAuth2` `access_token`; kept as `token` for older clients).
    pub token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user: AuthUser,
}

/// Session probe: either anonymous or authenticated with user summary.
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub authenticated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<AuthUser>,
}

impl SessionResponse {
    #[must_use]
    pub fn anonymous() -> Self {
        Self {
            authenticated: false,
            user: None,
        }
    }

    #[must_use]
    pub fn authenticated(user: AuthUser) -> Self {
        Self {
            authenticated: true,
            user: Some(user),
        }
    }
}
