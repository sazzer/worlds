mod get_user;
mod parse;
mod save_user;

use std::sync::Arc;

pub use save_user::SaveUserError;

use crate::database::Database;

/// Repository of user records.
pub struct UserRepository {
    database: Arc<Database>,
}

impl UserRepository {
    /// Create a new user repository.
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
