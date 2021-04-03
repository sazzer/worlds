mod email;
mod user_id;
mod username;

use crate::model::Resource;
pub use email::*;
pub use user_id::*;
pub use username::*;

/// The data representing a user.
pub struct UserData {
    pub username: Username,
    pub email: Email,
    pub display_name: String,
}

/// The resource representing a persisted user.
pub type UserResource = Resource<UserID, UserData>;
