use super::model::UserModel;
use crate::http::{Response, SimpleRespondable};
use actix_http::http::StatusCode;
use actix_web::http::header::{ETag, EntityTag};

pub async fn handle() -> Response<SimpleRespondable<UserModel>> {
    let user = UserModel {
        username: "sazzer".to_owned(),
        email: "graham@grahamcox.co.uk".to_owned(),
        display_name: "Graham".to_owned(),
    };

    SimpleRespondable::new(user)
        .with_status_code(StatusCode::OK)
        .with_header(ETag(EntityTag::strong("hello".to_owned())))
        .into()
}
