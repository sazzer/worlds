use super::UserRepository;
use crate::{
    model::Identity,
    users::{UserData, UserId, UserResource},
};

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
            .map(|row| UserResource {
                identity: Identity {
                    id: row.get("user_id"),
                    version: row.get("version"),
                    created: row.get("created"),
                    updated: row.get("updated"),
                },
                data: UserData {
                    username: row.get("username"),
                    email: row.get("email"),
                    display_name: row.get("display_name"),
                },
            })
    }
}
