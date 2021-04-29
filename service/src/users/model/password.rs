use argonautica::{Hasher, Verifier};
use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};

/// Wrapper around a hashed password.
#[derive(FromSql)]
pub struct Password(String);

impl Password {
    /// Hash a plaintext password into a `Password` object.
    ///
    /// # Parameters
    /// - `input` - The plaintext password to hash
    ///
    /// # Returns
    /// The hashed version.
    pub fn from_plaintext(input: &str) -> Password {
        let hash = Hasher::default().with_password(input).opt_out_of_secret_key(true).hash().unwrap();

        Password(hash)
    }
}

impl ToSql for Password {
    accepts!(TEXT, VARCHAR);
    to_sql_checked!();

    fn to_sql(&self, t: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password(Redacted)")
    }
}

impl PartialEq<&str> for Password {
    fn eq(&self, other: &&str) -> bool {
        Verifier::default()
            .with_hash(&self.0)
            .with_password(*other)
            .verify()
            .unwrap_or_else(|e| {
                tracing::warn!(e = ?e, "Failed to verify password");

                false
            })
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use super::*;

    #[test]
    fn debug() {
        let password = Password::from_plaintext("hello");
        let formatted = format!("{:?}", password);

        check!(formatted == "Password(Redacted)");
    }

    #[test]
    fn verify() {
        let password = Password::from_plaintext("hello");

        check!(password == "hello");
        check!(password != "Hello");
        check!(password != "hell0");
        check!(password != "hello ");
        check!(password != " hello");
        check!(password != " hello ");
    }
}
