mod authorize;
mod constants;
mod generate;

use biscuit::jws::Secret;

/// Service for authorizing users.
pub struct AuthorizationService {
    secret: Secret,
}

impl AuthorizationService {
    /// Create a new authorization service.
    pub fn new(secret: &str) -> Self {
        Self {
            secret: Secret::Bytes(secret.to_owned().into_bytes()),
        }
    }
}
