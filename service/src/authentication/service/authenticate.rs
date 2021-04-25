use super::AuthenticationService;
use crate::{
    authorization::{AccessToken, SecurityContext},
    users::Username,
};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthenticateError {
    #[error("Unknown user")]
    UnknownUser,

    #[error("Invaid password")]
    InvalidPassword,
}

impl AuthenticationService {
    /// Attempt to authenticate the provided username and password.
    ///
    /// # Parameters
    /// - `username` - The username to authenticate
    /// - `password` - The password to authenticate
    ///
    /// # Returns
    /// A security context and access token for the credentials, or else an error if authentication
    /// failed.
    pub async fn authenticate(&self, username: &Username, password: &str) -> Result<(SecurityContext, AccessToken), AuthenticateError> {
        let user = self.users_service.get_user_by_username(username).await;

        match user {
            None => Err(AuthenticateError::UnknownUser),
            Some(u) if u.data.password != password => Err(AuthenticateError::InvalidPassword),
            Some(u) => {
                let (security_context, access_token) = self.authorization_service.generate_security_context(u.identity.id.into());

                Ok((security_context, access_token))
            },
        }
    }
}
