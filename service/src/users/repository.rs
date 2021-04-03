pub mod get_user;

use crate::database::Database;
use std::sync::Arc;

/// Repository of user records.
pub struct Repository {
    /// The database connection.
    database: Arc<Database>,
}

impl Repository {
    /// Create a new user repository.
    ///
    /// # Parameters
    /// - `database` - The database connection
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
