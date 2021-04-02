use std::str::FromStr;

/// Representation of an access token as read from an Authorization header.
#[derive(Debug, PartialEq)]
pub struct AccessToken(pub String);

/// Potential errors from parsing an access token from a header string.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AccessTokenParseError {
    #[error("The provided authorization was not a bearer token")]
    NonBearerToken,
    #[error("The provided bearer token was blank")]
    Blank,
}

impl FromStr for AccessToken {
    type Err = AccessTokenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix("Bearer ").map(str::trim).map_or(Err(Self::Err::NonBearerToken), |token| {
            if token.is_empty() {
                Err(Self::Err::Blank)
            } else {
                Ok(AccessToken(token.to_owned()))
            }
        })
    }
}

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("Bearer someToken", AccessToken("someToken".to_owned()) ; "Simple token")]
    #[test_case("Bearer    someToken", AccessToken("someToken".to_owned()) ; "Left padded")]
    #[test_case("Bearer someToken   ", AccessToken("someToken".to_owned()) ; "Right padded")]
    #[test_case("Bearer    someToken   ", AccessToken("someToken".to_owned()) ; "Both padded")]
    fn parse_success(input: &str, expected: AccessToken) {
        let result: Result<AccessToken, AccessTokenParseError> = input.parse();

        let_assert!(Ok(token) = result);
        check!(expected == token);
    }

    #[test_case("NonBearer someToken", AccessTokenParseError::NonBearerToken ; "Non-Bearer Token")]
    #[test_case("Bearer ", AccessTokenParseError::Blank ; "Missing Token")]
    #[test_case("Bearer     ", AccessTokenParseError::Blank ; "Whitespace Token")]
    fn parse_failure(input: &str, expected: AccessTokenParseError) {
        let result: Result<AccessToken, AccessTokenParseError> = input.parse();

        let_assert!(Err(err) = result);
        check!(expected == err);
    }
}
