use std::str::FromStr;

use crate::http::hal::Link;

/// Representation of a User ID.
#[derive(Debug, PartialEq)]
pub struct UserID(String);

/// Errors from parsing a User ID.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseUserIDError {
    #[error("User ID was blank")]
    Blank,
}

impl FromStr for UserID {
    type Err = ParseUserIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            Err(ParseUserIDError::Blank)
        } else {
            Ok(UserID(trimmed.to_owned()))
        }
    }
}

impl From<UserID> for Link {
    fn from(user_id: UserID) -> Self {
        Link::from(format!("/users/{}", user_id.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("123", UserID("123".to_owned()) ; "Simple String")]
    #[test_case("  123", UserID("123".to_owned()) ; "Left Padded")]
    #[test_case("123  ", UserID("123".to_owned()) ; "Right Padded")]
    #[test_case("  123  ", UserID("123".to_owned()) ; "Both Padded")]
    fn parse_success(input: &str, expected: UserID) {
        let result: Result<UserID, ParseUserIDError> = input.parse();

        let_assert!(Ok(user_id) = result);
        check!(user_id == expected);
    }

    #[test_case("", ParseUserIDError::Blank ; "Empty String")]
    #[test_case("   ", ParseUserIDError::Blank ; "Whitespace String")]
    fn parse_error(input: &str, expected: ParseUserIDError) {
        let result: Result<UserID, ParseUserIDError> = input.parse();

        let_assert!(Err(e) = result);
        check!(e == expected);
    }
}
