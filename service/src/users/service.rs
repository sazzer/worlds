mod create_user;
mod get_user;
mod update_user;

pub use create_user::CreateUserError;

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
