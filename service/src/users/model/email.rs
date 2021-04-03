use std::str::FromStr;

/// Representation of an email address.
#[derive(Debug, PartialEq)]
pub struct Email(String);

/// Errors from parsing an email address.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseEmailError {
    #[error("Email was blank")]
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

    #[test_case("123", Email("123".to_owned()) ; "Simple String")]
    #[test_case("  123", Email("123".to_owned()) ; "Left Padded")]
    #[test_case("123  ", Email("123".to_owned()) ; "Right Padded")]
    #[test_case("  123  ", Email("123".to_owned()) ; "Both Padded")]
    fn parse_success(input: &str, expected: Email) {
        let result: Result<Email, ParseEmailError> = input.parse();

        let_assert!(Ok(email) = result);
        check!(email == expected);
    }

    #[test_case("", ParseEmailError::Blank ; "Empty String")]
    #[test_case("   ", ParseEmailError::Blank ; "Whitespace String")]
    fn parse_error(input: &str, expected: ParseEmailError) {
        let result: Result<Email, ParseEmailError> = input.parse();

        let_assert!(Err(e) = result);
        check!(e == expected);
    }
}
