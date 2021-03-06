use std::str::FromStr;

use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

use crate::http::valid::Validatable;

/// The email address of a user.
#[derive(Debug, PartialEq, Deserialize, Serialize, FromSql)]
pub struct Email(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseEmailError {
    #[error("The email address was blank")]
    Blank,
}

impl FromStr for Email {
    type Err = ParseEmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(ParseEmailError::Blank)
        } else {
            Ok(Email(trimmed.to_owned()))
        }
    }
}

impl ToSql for Email {
    accepts!(TEXT, VARCHAR);
    to_sql_checked!();

    fn to_sql(&self, t: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

impl Validatable for Email {
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "string",
            "minLength": 1,
            "pattern": "^[^@]+@[^@]+\\.[^@\\.]+$"
        })
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use test_case::test_case;

    use super::*;

    #[test_case("testuser@example.com", "testuser@example.com" ; "Simple")]
    #[test_case("   testuser@example.com", "testuser@example.com" ; "Left padded")]
    #[test_case("testuser@example.com   ", "testuser@example.com" ; "Right padded")]
    #[test_case("   testuser@example.com   ", "testuser@example.com" ; "Both padded")]
    fn test_parse_success(input: &str, expected: &str) {
        let result: Result<Email, ParseEmailError> = input.parse();

        let_assert!(Ok(output) = result);
        let_assert!(Email(value) = output);
        check!(value == expected);
    }

    #[test_case("", &ParseEmailError::Blank ; "Blank")]
    #[test_case("   ", &ParseEmailError::Blank ; "Whitespace")]
    fn test_parse_fail(input: &str, expected: &ParseEmailError) {
        let result: Result<Email, ParseEmailError> = input.parse();

        let_assert!(Err(e) = result);
        check!(&e == expected);
    }
}
