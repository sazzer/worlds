use tokio_postgres::error::{DbError, SqlState};

use super::UserRepository;
use crate::users::UserResource;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SaveUserError {
    #[error("Duplicate username")]
    DuplicateUsername,

    #[error("An unknown error occurred")]
    UnknownError,
}

impl UserRepository {
    /// Create a new user record from the provided User Resource.
    ///
    /// # Parameters
    /// - `user` - The details of the user to create.
    ///
    /// # Returns
    /// The created user resource.
    #[tracing::instrument(skip(self))]
    pub async fn create_user(&self, user: &UserResource) -> Result<UserResource, SaveUserError> {
        let conn = self.database.connect().await;

        let created: UserResource = conn.query_one("INSERT INTO users(user_id, version, created, updated, username, display_name, email, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *", 
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
            .map(|row| row.into())?;

        Ok(created)
    }
}

impl From<tokio_postgres::Error> for SaveUserError {
    fn from(e: tokio_postgres::Error) -> Self {
        let mut result = None;

        if e.code() == Some(&SqlState::UNIQUE_VIOLATION) {
            let db_error: Option<DbError> = e.into_source().and_then(|e| e.downcast_ref::<DbError>().cloned());

            result = db_error
                .and_then(|e| e.constraint().map(std::borrow::ToOwned::to_owned))
                .map(|constraint| {
                    if constraint == "users_username_key" {
                        SaveUserError::DuplicateUsername
                    } else {
                        tracing::warn!("Unexpected constraint violation error: {:?}", constraint);
                        SaveUserError::UnknownError
                    }
                });
        } else {
            tracing::warn!("Unexpected database error: {:?}", e);
        }

        result.unwrap_or(SaveUserError::UnknownError)
    }
}
