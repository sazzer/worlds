use std::str::FromStr;

use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use uuid::Uuid;

use crate::authorization::Principal;

/// The ID of a user.
#[derive(Debug, PartialEq, Serialize, FromSql)]
pub struct UserId(Uuid);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseUserIdError {
    #[error("The User ID was blank")]
    Blank,

    #[error("The User ID was malformed")]
    Malformed,
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for UserId {
    type Err = ParseUserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(ParseUserIdError::Blank)
        } else {
            let uuid = Uuid::parse_str(trimmed).map_err(|e| {
                tracing::warn!(e = ?e, "Failed to parse User ID as UUID");
                ParseUserIdError::Malformed
            })?;

            Ok(UserId(uuid))
        }
    }
}

impl ToSql for UserId {
    accepts!(UUID);
    to_sql_checked!();

    fn to_sql(&self, t: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

impl From<UserId> for Principal {
    fn from(user_id: UserId) -> Self {
        Self::User(user_id.0.to_string())
    }
}

impl From<&UserId> for Principal {
    fn from(user_id: &UserId) -> Self {
        Self::User(user_id.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use test_case::test_case;

    use super::*;

    #[test_case("50b44401-a345-419d-a8a8-baf22df76c05", "50b44401-a345-419d-a8a8-baf22df76c05" ; "Simple")]
    #[test_case("50B44401-A345-419D-A8A8-BAF22DF76C05", "50b44401-a345-419d-a8a8-baf22df76c05" ; "Capitals")]
    #[test_case("   50b44401-a345-419d-a8a8-baf22df76c05", "50b44401-a345-419d-a8a8-baf22df76c05" ; "Left padded")]
    #[test_case("50b44401-a345-419d-a8a8-baf22df76c05   ", "50b44401-a345-419d-a8a8-baf22df76c05" ; "Right padded")]
    #[test_case("   50b44401-a345-419d-a8a8-baf22df76c05   ", "50b44401-a345-419d-a8a8-baf22df76c05" ; "Both padded")]
    fn test_parse_success(input: &str, expected: &str) {
        let result: Result<UserId, ParseUserIdError> = input.parse();

        let_assert!(Ok(output) = result);
        let_assert!(UserId(value) = output);
        check!(value.to_string() == expected);
    }

    #[test_case("", &ParseUserIdError::Blank ; "Blank")]
    #[test_case("   ", &ParseUserIdError::Blank ; "Whitespace")]
    #[test_case("xxx", &ParseUserIdError::Malformed ; "Malformed")]
    fn test_parse_fail(input: &str, expected: &ParseUserIdError) {
        let result: Result<UserId, ParseUserIdError> = input.parse();

        let_assert!(Err(e) = result);
        check!(&e == expected);
    }
}
