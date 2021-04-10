use serde::Serialize;
use std::str::FromStr;

/// The email address of a user.
#[derive(Debug, PartialEq, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("testuser@example.com", "testuser@example.com" ; "Simple")]
    #[test_case("   testuser@example.com", "testuser@example.com" ; "Left padded")]
    #[test_case("testuser@example.com   ", "testuser@example.com" ; "Right padded")]
    #[test_case("   testuser@example.com   ", "testuser@example.com" ; "Both padded")]
    fn test_parse_success(input: &str, expected: &str) {
        let result: Result<Email, ParseEmailError> = input.parse();

        let_assert!(Ok(output) = result);
        let_assert!(Email(uuid) = output);
        check!(uuid.to_string() == expected);
    }

    #[test_case("", ParseEmailError::Blank ; "Blank")]
    #[test_case("   ", ParseEmailError::Blank ; "Whitespace")]
    fn test_parse_fail(input: &str, expected: ParseEmailError) {
        let result: Result<Email, ParseEmailError> = input.parse();

        let_assert!(Err(e) = result);
        check!(e == expected);
    }
}
