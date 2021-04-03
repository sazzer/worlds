use argonautica::{Hasher, Verifier};
use std::fmt::{Debug, Formatter};

/// Representation of a hashed password.
#[derive(PartialEq)]
pub struct Password(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum HashPasswordError {
    #[error("The password was blank")]
    Blank,

    #[error("An unexpected error occurred hashing the password")]
    Unexpected,
}

impl Password {
    pub fn new_from_plaintext(plaintext: &str) -> Result<Self, HashPasswordError> {
        if plaintext.is_empty() {
            return Err(HashPasswordError::Blank);
        }

        let mut hasher = Hasher::default();
        hasher.opt_out_of_secret_key(true);
        hasher.with_password(plaintext);

        hasher
            .hash()
            .map_err(|e| {
                tracing::warn!(e = ?e, "Failed to hash password");
                HashPasswordError::Unexpected
            })
            .map(Password)
    }

    /// Construct a new `Password` object from the provided hash.
    ///
    /// # Parameters
    /// - `hash` - The hashed password
    pub fn new_from_hash(hash: &str) -> Self {
        Password(hash.to_owned())
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password{{Redacted}}")
    }
}

impl PartialEq<&str> for Password {
    fn eq(&self, other: &&str) -> bool {
        Verifier::default().with_hash(&self.0).with_password(*other).verify().unwrap_or_else(|e| {
            tracing::warn!(e = ?e, "Failed to verify password");
            false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};

    #[test]
    fn test_hash() {
        let result = Password::new_from_plaintext("password");
        let_assert!(Ok(_) = result);
    }

    #[test]
    fn test_hash_blank() {
        let result = Password::new_from_plaintext("");
        let_assert!(Err(e) = result);
        check!(e == HashPasswordError::Blank);
    }

    #[test]
    fn test_verify() {
        let result = Password::new_from_plaintext("password");
        let_assert!(Ok(password) = result);

        check!(password == "password");
        check!(password != "password2");
        check!(password != "passwor");
        check!(password != "Password");
        check!(password != "password ");
    }
}
