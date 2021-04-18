use crate::{
    http::{
        model::ResourceResponse,
        response::{Response, SimpleRespondable},
    },
    users::{Email, UserId, UserResource, Username},
};
use actix_web::http::header::CacheDirective;
use serde::Serialize;

/// Full representation of a user on the HTTP API.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullUserModel {
    pub user_id: UserId,
    pub username: Username,
    pub email: Email,
    pub display_name: String,
}

/// Simple representation of a user on the HTTP API.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUserModel {
    pub user_id: UserId,
    pub username: Username,
    pub display_name: String,
}

impl From<UserResource> for FullUserModel {
    fn from(user: UserResource) -> Self {
        Self {
            user_id: user.identity.id,
            username: user.data.username,
            email: user.data.email,
            display_name: user.data.display_name,
        }
    }
}

impl From<UserResource> for SimpleUserModel {
    fn from(user: UserResource) -> Self {
        Self {
            user_id: user.identity.id,
            username: user.data.username,
            display_name: user.data.display_name,
        }
    }
}

impl ResourceResponse for UserResource {
    fn cache_control(&self) -> Option<Vec<CacheDirective>> {
        Some(vec![CacheDirective::Private, CacheDirective::MaxAge(3600)])
    }
}

pub type FullUserResponse = Response<SimpleRespondable<FullUserModel>>;
pub type SimpleUserResponse = Response<SimpleRespondable<SimpleUserModel>>;
