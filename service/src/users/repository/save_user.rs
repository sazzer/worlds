use chrono::Utc;
use tokio_postgres::error::{DbError, SqlState};
use uuid::Uuid;

use super::UserRepository;
use crate::{
    model::Identity,
    users::{UserData, UserId, UserResource},
};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SaveUserError {
    #[error("Duplicate username")]
    DuplicateUsername,

    #[error("The user was not found")]
    UnknownUser,

    #[error("An unknown error occurred")]
    UnknownError,
}

impl UserRepository {
    /// Create a new user record from the provided User data.
    ///
    /// # Parameters
    /// - `user` - The details of the user to create.
    ///
    /// # Returns
    /// The created user resource.
    #[tracing::instrument(skip(self))]
    pub async fn create_user(&self, user: &UserData) -> Result<UserResource, SaveUserError> {
        let conn = self.database.connect().await;

        let identity = Identity::<UserId>::default();

        let created: UserResource = conn.query_one("INSERT INTO users(user_id, version, created, updated, username, display_name, email, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *", 
        &[
          &identity.id,
          &identity.version,
          &identity.created,
          &identity.updated,
          &user.username,
          &user.display_name,
          &user.email,
          &user.password,
          ])
            .await
            .map(|row| row.into())?;

        Ok(created)
    }

    /// Update an existing user record from the provided User data.
    ///
    /// # Parameters
    /// - `user` - The details of the user to create.
    ///
    /// # Returns
    /// The updated user resource.
    #[tracing::instrument(skip(self))]
    pub async fn update_user(&self, id: &UserId, data: &UserData) -> Result<UserResource, SaveUserError> {
        let conn = self.database.connect().await;

        let version = Uuid::new_v4();
        let updated = Utc::now();

        conn.query_opt("UPDATE users SET version = $2, updated = $3, username = $4, display_name = $5, email = $6, password = $7 WHERE user_id = $1 RETURNING *", 
        &[
          &id,
          &version,
          &updated,
          &data.username,
          &data.display_name,
          &data.email,
          &data.password,
          ])
            .await?
            .ok_or(SaveUserError::UnknownUser)
            .map(|row| row.into())
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
