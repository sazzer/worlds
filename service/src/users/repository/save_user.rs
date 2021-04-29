use super::UserRepository;
use crate::users::UserResource;

impl UserRepository {
    /// Create a new user record from the provided User Resource.
    ///
    /// # Parameters
    /// - `user` - The details of the user to create.
    ///
    /// # Returns
    /// The created user resource.
    #[tracing::instrument(skip(self))]
    pub async fn create_user(&self, user: &UserResource) -> Result<UserResource, ()> {
        let conn = self.database.connect().await;
        conn.query_one("INSERT INTO users(user_id, version, created, updated, username, display_name, email, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *", 
        &[
          &user.identity.id,
          &user.identity.version,
          &user.identity.created,
          &user.identity.updated,
          &user.data.username,
          &user.data.display_name,
          &user.data.email,
          &user.data.password,
          ])
            .await
            .map(|row| row.into())
            .map_err(|e| {
              tracing::warn!(e = ?e, "Failed to create user record");
            })
    }
}
