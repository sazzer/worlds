use std::fmt::{Display, Formatter};

use actix_http::http::StatusCode;

use super::{Problem, ProblemType, ProblemTypeStatus};

/// A simple representation of a problem type.
#[derive(Debug)]
pub struct SimpleProblemType {
    /// The actual problem code
    pub problem_type:  &'static str,
    /// The title of the problem
    pub problem_title: &'static str,
    /// The default status code for the problem
    pub status_code:   StatusCode,
}

impl ProblemType for SimpleProblemType {
    /// Determine the value to use for the problem type.
    fn problem_type(&self) -> &'static str {
        self.problem_type
    }
}

impl ProblemTypeStatus for SimpleProblemType {
    /// Determine the status code for the problem.
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl Display for SimpleProblemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.problem_title)
    }
}

impl From<SimpleProblemType> for Problem {
    fn from(problem_type: SimpleProblemType) -> Self {
        Self::new(problem_type)
    }
}

/// Problem to indicate that a resource was not found.
pub const NOT_FOUND: SimpleProblemType = SimpleProblemType {
    problem_type:  "about:blank",
    problem_title: "The requested resource was not found",
    status_code:   StatusCode::NOT_FOUND,
};

/// Problem to indicate that a request was unauthorized.
pub const UNAUTHORIZED: SimpleProblemType = SimpleProblemType {
    problem_type:  "about:blank",
    problem_title: "Unauthorized",
    status_code:   StatusCode::UNAUTHORIZED,
};

/// Problem to indicate that a request was forbidden.
pub const FORBIDDEN: SimpleProblemType = SimpleProblemType {
    problem_type:  "about:blank",
    problem_title: "Forbidden",
    status_code:   StatusCode::FORBIDDEN,
};

/// Problem to indicate that a request was a bad request.
pub const BAD_REQUEST: SimpleProblemType = SimpleProblemType {
    problem_type:  "about:blank",
    problem_title: "Bad Request",
    status_code:   StatusCode::BAD_REQUEST,
};

/// Problem to indicate that a request was a valid request but wasn't processable for this request.
pub const UNPROCESSABLE_ENTITY: SimpleProblemType = SimpleProblemType {
    problem_type:  "about:blank",
    problem_title: "Unprocessable Entity",
    status_code:   StatusCode::UNPROCESSABLE_ENTITY,
};

/// Problem to indicate that an internal server error occurred.
pub const INTERNAL_SERVER_ERROR: SimpleProblemType = SimpleProblemType {
    problem_type:  "about:blank",
    problem_title: "Internal Server Error",
    status_code:   StatusCode::INTERNAL_SERVER_ERROR,
};
