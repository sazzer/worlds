use super::AuthenticationService;
use crate::{
    authorization::{AccessToken, SecurityContext},
    users::{Email, Password, UserData, Username},
};

/// Details needed to register a new user.
#[derive(Debug)]
pub struct Registration {
    pub username:     Username,
    pub email:        Email,
    pub display_name: String,
    pub password:     Password,
}

/// Errors that can happen when registering a user
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RegistrationError {
    #[error("The username is already registered")]
    DuplicateUsername,
}

impl AuthenticationService {
    /// Attempt to register a new user account.
    ///
    /// # Parameters
    /// - `registration` - The details to register the user with
    ///
    /// # Returns
    /// The authentication details for the new user.
    pub async fn register(&self, registration: Registration) -> Result<(SecurityContext, AccessToken), RegistrationError> {
        let user = self.users_service.create_user(registration.into()).await.unwrap();

        let (security_context, access_token) = self.authorization_service.generate_security_context(user.identity.id.into());

        Ok((security_context, access_token))
    }
}

impl From<Registration> for UserData {
    fn from(registration: Registration) -> Self {
        Self {
            username:     registration.username,
            email:        registration.email,
            display_name: registration.display_name,
            password:     registration.password,
        }
    }
}
