use serde::Serialize;
use std::str::FromStr;

/// The Username of a user.
#[derive(Debug, PartialEq, Serialize)]
pub struct Username(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseUsernameError {
    #[error("The username was blank")]
    Blank,
}

impl FromStr for Username {
    type Err = ParseUsernameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(ParseUsernameError::Blank)
        } else {
            Ok(Username(trimmed.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("testUsername", "testUsername" ; "Simple")]
    #[test_case("   testUsername", "testUsername" ; "Left padded")]
    #[test_case("testUsername   ", "testUsername" ; "Right padded")]
    #[test_case("   testUsername   ", "testUsername" ; "Both padded")]
    fn test_parse_success(input: &str, expected: &str) {
        let result: Result<Username, ParseUsernameError> = input.parse();

        let_assert!(Ok(output) = result);
        let_assert!(Username(uuid) = output);
        check!(uuid.to_string() == expected);
    }

    #[test_case("", ParseUsernameError::Blank ; "Blank")]
    #[test_case("   ", ParseUsernameError::Blank ; "Whitespace")]
    fn test_parse_fail(input: &str, expected: ParseUsernameError) {
        let result: Result<Username, ParseUsernameError> = input.parse();

        let_assert!(Err(e) = result);
        check!(e == expected);
    }
}
