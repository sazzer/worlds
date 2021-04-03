use super::Repository;
use crate::users::{UserResource, Username};

impl Repository {
    /// Get the single user that matches the provided username.
    ///
    /// # Parameters
    /// - `username` - The username to query for.
    ///
    /// # Returns
    /// The matching user, or `None` if one wasn't found.
    pub async fn get_user_by_username(&self, username: &Username) -> Option<UserResource> {
        let conn = self.database.connect().await;
        let result = conn
            .query_opt("SELECT * FROM users WHERE username = $1", &[username])
            .await
            .unwrap_or_else(|e| {
                tracing::warn!(e = ?e, username = ?username, "Error querying for user");
                None
            });

        None
    }
}
