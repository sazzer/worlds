use chrono::{DateTime, Utc};

/// An authenticated principal.
#[derive(Debug, PartialEq)]
pub enum Principal {
    /// An authenticated user principal.
    User(String),
}

/// An authenticated security context.
#[derive(Debug)]
pub struct SecurityContext {
    /// The principal that was authenticated.
    pub principal: Principal,
    /// When the security context was issued.
    pub issued: DateTime<Utc>,
    /// When the security context expires.
    pub expires: DateTime<Utc>,
}

/// An access token, representing a signed security context.
#[derive(Debug)]
pub struct AccessToken(pub(super) String);
