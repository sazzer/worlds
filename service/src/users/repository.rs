mod get_user;
mod parse;

use std::sync::Arc;

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
