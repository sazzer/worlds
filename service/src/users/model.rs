mod email;
mod password;
mod user_id;
mod username;

pub use email::*;
pub use password::*;
pub use user_id::*;
pub use username::*;

use crate::model::Resource;

/// The data representing a user.
#[derive(Debug)]
pub struct UserData {
    pub username:     Username,
    pub email:        Email,
    pub display_name: String,
    pub password:     Password,
}

/// Type representing a persisted user.
pub type UserResource = Resource<UserId, UserData>;
