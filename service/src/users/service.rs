mod get_user;

use super::repository::UserRepository;

/// Service layer for working with users.
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    /// Create a new user service.
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
}
