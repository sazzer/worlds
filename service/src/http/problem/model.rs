use actix_http::http::StatusCode;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

/// Trait to represent the type of problem.
pub trait ProblemType: Debug + Display {
    /// A URI Reference that identifies the problem type.
    fn problem_type(&self) -> &'static str;
}

/// Trait that problem type instances can implement that mean they define their own status codes.
pub trait ProblemTypeStatus {
    /// The status code to use for this problem type
    fn status_code(&self) -> StatusCode;
}

/// Representation of an RFC-7807 Problem.
#[derive(Debug)]
pub struct Problem {
    /// The actual error that occurred
    pub error: Box<dyn ProblemType>,
    /// The HTTP Status code to use
    pub status: StatusCode,
    /// An additional detail message
    pub detail: Option<String>,
    /// An additional instance subtype
    pub instance: Option<String>,
    /// Any extra details
    pub extra: HashMap<String, Value>,
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Problem {
    /// Create a new Problem instance
    ///
    /// # Parameters
    /// - `error` - The error code
    /// - `status` - The HTTP Status code
    ///
    /// # Returns
    /// The problem
    pub fn new_with_status<T>(error: T, status: StatusCode) -> Self
    where
        T: ProblemType + 'static,
    {
        Self {
            error: Box::new(error),
            status,
            detail: None,
            instance: None,
            extra: HashMap::new(),
        }
    }

    /// Create a new Problem instance
    ///
    /// # Parameters
    /// - `error` - The error code
    ///
    /// # Returns
    /// The problem
    pub fn new<T>(error: T) -> Self
    where
        T: ProblemType + ProblemTypeStatus + 'static,
    {
        let status = error.status_code();
        Self::new_with_status(error, status)
    }

    /// Set the Detail of the Problem instance
    ///
    /// # Parameters
    /// - `detail` - The detail to use
    #[allow(dead_code)]
    pub fn with_detail<S>(self, detail: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            detail: Some(detail.into()),
            ..self
        }
    }

    /// Set the Instance of the Problem instance
    ///
    /// # Parameters
    /// - `instance` - The instance value to use
    #[allow(dead_code)]
    pub fn with_instance<S>(self, instance: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            instance: Some(instance.into()),
            ..self
        }
    }

    /// Set some extra data on the Problem instance
    ///
    /// # Parameters
    /// - `key` - The key of the extra data
    /// - `value` - The value of the extra data
    pub fn with_extra<K, V>(self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        let mut extra = self.extra;
        extra.insert(
            key.into(),
            serde_json::to_value(value).expect("Failed to serialize extra detail"),
        );

        Self { extra, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum ProblemDetails {
        SomeProblem,
    }

    impl ProblemType for ProblemDetails {
        fn problem_type(&self) -> &'static str {
            "tag:new_landing,2021:some/problem"
        }
    }

    impl Display for ProblemDetails {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Some Problem")
        }
    }

    impl ProblemTypeStatus for ProblemDetails {
        fn status_code(&self) -> StatusCode {
            StatusCode::NO_CONTENT
        }
    }

    #[test]
    fn test_basic_problem_with_status() {
        let problem =
            Problem::new_with_status(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST);

        assert_eq!(StatusCode::BAD_REQUEST, problem.status);
        assert_eq!(
            "tag:new_landing,2021:some/problem",
            problem.error.problem_type()
        );
        assert_eq!(None, problem.detail);
        assert_eq!(None, problem.instance);
        assert_eq!(0, problem.extra.len());
    }

    #[test]
    fn test_basic_problem() {
        let problem = Problem::new(ProblemDetails::SomeProblem);

        assert_eq!(StatusCode::NO_CONTENT, problem.status);
        assert_eq!(
            "tag:new_landing,2021:some/problem",
            problem.error.problem_type()
        );
        assert_eq!(None, problem.detail);
        assert_eq!(None, problem.instance);
        assert_eq!(0, problem.extra.len());
    }

    #[test]
    fn test_full_problem() {
        let problem =
            Problem::new_with_status(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST)
                .with_detail("Some Detail")
                .with_instance("Some Instance")
                .with_extra("some_key", "Some Value")
                .with_extra("other_key", 42);

        assert_eq!(StatusCode::BAD_REQUEST, problem.status);
        assert_eq!(
            "tag:new_landing,2021:some/problem",
            problem.error.problem_type()
        );
        assert_eq!(Some("Some Detail".to_owned()), problem.detail);
        assert_eq!(Some("Some Instance".to_owned()), problem.instance);
        assert_eq!(2, problem.extra.len());
        assert_eq!(
            Some(&serde_json::to_value("Some Value").unwrap()),
            problem.extra.get(&"some_key".to_owned())
        );
        assert_eq!(
            Some(&serde_json::to_value(42).unwrap()),
            problem.extra.get(&"other_key".to_owned())
        );
    }
}
