use crate::{
    authorization::{Authentication, Principal},
    home::LinkContributor,
    http::hal::Link,
    users::{ParseUserIDError, UserID},
};

/// Means to generate the links for the home document.
pub struct HomeLinks {}

impl LinkContributor for HomeLinks {
    fn generate_links(&self, authentication: &Authentication) -> Vec<(String, Link)> {
        let mut links = vec![];

        if let Authentication::Authenticated(security_context) = authentication {
            if let Principal::User(user_id) = &security_context.principal {
                let user_id: Result<UserID, ParseUserIDError> = user_id.parse();

                if let Ok(user_id) = user_id {
                    links.push(("tag:worlds,2021:rels/user".to_owned(), user_id.into()))
                }
            }
        }

        links
    }
}
