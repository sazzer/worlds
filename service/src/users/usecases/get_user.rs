use crate::users::{repository::Repository, UserResource, Username};
use std::sync::Arc;

/// Use case for getting user records.
pub struct GetUserUseCase {
    /// The repository to work with.
    repository: Arc<Repository>,
}

impl GetUserUseCase {
    /// Create a new use case.
    ///
    /// # Parameters
    /// - `repository` - The repository of user records
    pub fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }

    /// Get the user that has the provided username.
    ///
    /// # Parameters
    /// - `username` - The username to look up
    ///
    /// # Returns
    /// The user, or `None` if one wasn't found.
    #[tracing::instrument(skip(self))]
    pub async fn get_user_for_username(&self, username: &Username) -> Option<UserResource> {
        self.repository.get_user_by_username(&username).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdatabase::TestDatabase;
    use assert2::check;

    #[actix_rt::test]
    async fn get_unknown_user() {
        let _ = env_logger::try_init();

        let test_database = TestDatabase::new();
        let database = crate::database::component::new(&test_database.url).await;
        let users = crate::users::component::new(database.database);

        let username: Username = "unknown".parse().unwrap();
        let user = users.get_user.get_user_for_username(&username).await;
        check!(user.is_none());
    }
}
