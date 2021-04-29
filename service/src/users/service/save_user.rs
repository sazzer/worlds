use super::UserService;
use crate::{
    model::Identity,
    users::{UserData, UserResource},
};

impl UserService {
    /// Create a new user in the repositry from the provided user data.
    ///
    /// # Parameters
    /// - `user` - The user data to create the user from
    ///
    /// # Returns
    /// The newly created user.
    pub async fn create_user(&self, user: UserData) -> Result<UserResource, ()> {
        let user = UserResource {
            identity: Identity::default(),
            data:     user,
        };

        self.repository.create_user(&user).await
    }
}
