use super::Principal;
use chrono::{DateTime, Utc};

/// Security Context that a request has authenticated as.
#[derive(Debug, PartialEq)]
pub struct SecurityContext {
    /// The principal that has authenticated.
    pub principal: Principal,
    /// When the security context was issued.
    pub issued: DateTime<Utc>,
    /// When the security context expires.
    pub expires: DateTime<Utc>,
}
