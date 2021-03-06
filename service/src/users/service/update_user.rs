use super::UserService;
use crate::users::{repository::SaveUserError, UserData, UserId, UserResource};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UpdateUserError<E>
where
    E: std::fmt::Debug,
{
    #[error("Unknown user")]
    UnknownUser,

    #[error("Duplicate username")]
    DuplicateUsername,

    #[error("An error occurred updating the user data")]
    UpdateError(E),

    #[error("An unknown error occurred")]
    UnknownError,
}

impl UserService {
    /// Update the user that has the provided ID, using the provided lambda to perform the updates.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to update
    /// - `f` - The function to update the user details
    ///
    /// # Returns
    /// The newly updated user.
    pub async fn update_user_by_id<F, E>(&self, user_id: &UserId, f: F) -> Result<UserResource, UpdateUserError<E>>
    where
        F: FnOnce(UserData) -> Result<UserData, E>,
        E: std::fmt::Debug,
    {
        let user = self.repository.get_user_by_id(user_id).await.ok_or(UpdateUserError::UnknownUser)?;

        let data = f(user.data).map_err(UpdateUserError::UpdateError)?;

        let result = self.repository.update_user(&user_id, &data).await?;

        Ok(result)
    }
}

impl<E> From<SaveUserError> for UpdateUserError<E>
where
    E: std::fmt::Debug,
{
    fn from(e: SaveUserError) -> Self {
        match e {
            SaveUserError::DuplicateUsername => Self::DuplicateUsername,
            SaveUserError::UnknownUser => Self::UnknownUser,
            SaveUserError::UnknownError => Self::UnknownError,
        }
    }
}
