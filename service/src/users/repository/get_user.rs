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
    pub async fn get_user_by_id(&self, user_id: UserId) -> Option<UserResource> {
        Some(UserResource {
            identity: Identity::default(),
            data: UserData {
                username: "sazzer".parse().unwrap(),
                email: "graham@grahamcox.co.uk".parse().unwrap(),
                display_name: "Graham".to_owned(),
            },
        })
    }
}
