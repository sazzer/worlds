use super::UserService;
use crate::users::{UserId, UserResource};

impl UserService {
    /// Get the User Resource that has the provided User ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to fetch.
    ///
    /// # Returns
    /// The user resource, or `None` if it couldn't be found.
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_id(&self, user_id: UserId) -> Option<UserResource> {
        self.repository.get_user_by_id(user_id).await
    }
}
