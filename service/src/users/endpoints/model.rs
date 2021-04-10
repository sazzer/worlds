use crate::{
    http::model::ResourceResponse,
    users::{Email, UserId, UserResource, Username},
};
use actix_web::http::header::CacheDirective;
use serde::Serialize;

/// Representation of a user on the HTTP API.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    pub user_id: UserId,
    pub username: Username,
    pub email: Email,
    pub display_name: String,
}

impl From<UserResource> for UserModel {
    fn from(user: UserResource) -> Self {
        Self {
            user_id: user.identity.id,
            username: user.data.username,
            email: user.data.email,
            display_name: user.data.display_name,
        }
    }
}

impl ResourceResponse for UserResource {
    fn cache_control(&self) -> Option<Vec<CacheDirective>> {
        Some(vec![CacheDirective::Private, CacheDirective::MaxAge(3600)])
    }
}
