use actix_http::http::StatusCode;

use crate::http::problem::SimpleProblemType;

/// Problem to indicate registering a new user with a duplicate username..
pub const DUPLICATE_USERNAME: SimpleProblemType = SimpleProblemType {
    problem_type:  "tag:worlds,2021:problems/authentication/register/duplicate_username",
    problem_title: "Duplicate Username",
    status_code:   StatusCode::UNPROCESSABLE_ENTITY,
};
