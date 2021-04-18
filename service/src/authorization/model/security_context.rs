use super::Principal;
use chrono::{DateTime, Utc};

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
