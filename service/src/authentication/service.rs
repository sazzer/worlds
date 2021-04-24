use crate::{authorization::AuthorizationService, users::UserService};
use std::sync::Arc;

/// Service layer for authenticating users.
pub struct AuthenticationService {
    users_service: Arc<UserService>,
    authorization_service: Arc<AuthorizationService>,
}

impl AuthenticationService {
    /// Create a new authentication service.
    pub fn new(
        users_service: Arc<UserService>,
        authorization_service: Arc<AuthorizationService>,
    ) -> Self {
        Self {
            users_service,
            authorization_service,
        }
    }
}
