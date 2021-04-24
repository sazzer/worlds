use super::UserRepository;
use crate::users::{UserId, UserResource, Username};

impl UserRepository {
    /// Get the User Resource that has the provided User ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to fetch.
    ///
    /// # Returns
    /// The user resource, or `None` if it couldn't be found.
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_id(&self, user_id: &UserId) -> Option<UserResource> {
        let conn = self.database.connect().await;
        conn.query_opt("SELECT * FROM users WHERE user_id = $1", &[&user_id])
            .await
            .ok()?
            .map(|row| row.into())
    }

    /// Get the User Resource that has the provided Username.
    ///
    /// # Parameters
    /// - `username` - The username of the user to fetch.
    ///
    /// # Returns
    /// The user resource, or `None` if it couldn't be found.
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_username(&self, username: &Username) -> Option<UserResource> {
        let conn = self.database.connect().await;
        conn.query_opt("SELECT * FROM users WHERE username = $1", &[&username])
            .await
            .ok()?
            .map(|row| row.into())
    }
}
