use super::UserService;
use crate::users::{repository::SaveUserError, UserData, UserResource};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CreateUserError {
    #[error("Duplicate username")]
    DuplicateUsername,

    #[error("An unknown error occurred")]
    UnknownError,
}

impl UserService {
    /// Create a new user in the repositry from the provided user data.
    ///
    /// # Parameters
    /// - `user` - The user data to create the user from
    ///
    /// # Returns
    /// The newly created user.
    pub async fn create_user(&self, user: UserData) -> Result<UserResource, CreateUserError> {
        let user = self.repository.create_user(&user).await?;

        Ok(user)
    }
}

impl From<SaveUserError> for CreateUserError {
    fn from(e: SaveUserError) -> Self {
        match e {
            SaveUserError::DuplicateUsername => Self::DuplicateUsername,
            SaveUserError::UnknownUser => unreachable!("This error is impossible for creating new users"),
            SaveUserError::UnknownError => Self::UnknownError,
        }
    }
}
