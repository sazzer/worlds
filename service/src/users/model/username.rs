use std::str::FromStr;

/// Representation of a Username.
#[derive(Debug, PartialEq)]
pub struct Username(String);

/// Errors from parsing a Username.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseUsernameError {
    #[error("Username was blank")]
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

    #[test_case("123", Username("123".to_owned()) ; "Simple String")]
    #[test_case("  123", Username("123".to_owned()) ; "Left Padded")]
    #[test_case("123  ", Username("123".to_owned()) ; "Right Padded")]
    #[test_case("  123  ", Username("123".to_owned()) ; "Both Padded")]
    fn parse_success(input: &str, expected: Username) {
        let result: Result<Username, ParseUsernameError> = input.parse();

        let_assert!(Ok(username) = result);
        check!(username == expected);
    }

    #[test_case("", ParseUsernameError::Blank ; "Empty String")]
    #[test_case("   ", ParseUsernameError::Blank ; "Whitespace String")]
    fn parse_error(input: &str, expected: ParseUsernameError) {
        let result: Result<Username, ParseUsernameError> = input.parse();

        let_assert!(Err(e) = result);
        check!(e == expected);
    }
}
