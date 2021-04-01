use super::{AccessToken, Principal, SecurityContext};
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{Compact, Secret},
    ClaimsSet, Validation, ValidationOptions,
};
use std::ops::Deref;

/// Use case for authorizing a security context.
pub struct AuthorizeSecurityContextUseCase {
    secret: Secret,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthorizeSecurityContextError {
    #[error("The access token was invalid")]
    InvalidToken,
}

impl AuthorizeSecurityContextUseCase {
    /// Create a new instance of the use case.
    pub fn new(secret: &str) -> AuthorizeSecurityContextUseCase {
        let signing_secret = Secret::Bytes(secret.to_owned().into_bytes());

        AuthorizeSecurityContextUseCase { secret: signing_secret }
    }

    /// Authorize the provided access token and return the security context that it represents.
    ///
    /// # Parameters
    /// - `token` - The access token to authorize
    ///
    /// # Returns
    /// The authorized security context.
    pub fn authorize(&self, token: AccessToken) -> Result<SecurityContext, AuthorizeSecurityContextError> {
        let encoded = Compact::<ClaimsSet<()>, ()>::new_encoded(token.as_ref());
        let decoded = encoded.decode(&self.secret, SignatureAlgorithm::HS256).map_err(|e| {
            tracing::warn!(e = ?e, token = ?token, "Failed to decode access token");
            AuthorizeSecurityContextError::InvalidToken
        })?;

        decoded
            .validate(ValidationOptions {
                issuer: Validation::Validate("tag:worlds,2021:authorization".to_owned()),
                audience: Validation::Validate("tag:worlds,2021:authorization".to_owned()),
                ..ValidationOptions::default()
            })
            .map_err(|e| {
                tracing::warn!(e = ?e, token = ?decoded, "Token validation failed");
                AuthorizeSecurityContextError::InvalidToken
            })?;

        let payload = decoded.payload().map_err(|_| AuthorizeSecurityContextError::InvalidToken)?;

        let sub = payload.registered.subject.clone().ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "sub", "Missing field");
            AuthorizeSecurityContextError::InvalidToken
        })?;
        let iat = payload.registered.issued_at.ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "iat", "Missing field");
            AuthorizeSecurityContextError::InvalidToken
        })?;
        let exp = payload.registered.expiry.ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "exp", "Missing field");
            AuthorizeSecurityContextError::InvalidToken
        })?;

        Ok(SecurityContext {
            principal: Principal::User(sub),
            issued: iat.deref().clone(),
            expires: exp.deref().clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use biscuit::{jws::RegisteredHeader, RegisteredClaims, SingleOrMultiple};
    use chrono::{DateTime, Duration, SubsecRound, Utc};
    use test_case::test_case;

    fn build_token(
        sub: Option<&str>,
        iss: Option<&str>,
        aud: Option<&str>,
        iat: Option<DateTime<Utc>>,
        exp: Option<DateTime<Utc>>,
        secret: &str,
    ) -> AccessToken {
        let decoded = Compact::new_decoded(
            RegisteredHeader {
                algorithm: SignatureAlgorithm::HS256,
                ..Default::default()
            }
            .into(),
            ClaimsSet::<()> {
                registered: RegisteredClaims {
                    issuer: iss.map(|s| s.parse().unwrap()),
                    subject: sub.map(|s| s.parse().unwrap()),
                    audience: aud.map(|s| SingleOrMultiple::Single(s.parse().unwrap())),
                    issued_at: iat.map(|t| t.into()),
                    expiry: exp.map(|t| t.into()),
                    ..Default::default()
                },
                private: (),
            },
        );

        let signing_secret = Secret::Bytes(secret.to_owned().into_bytes());
        let encoded = decoded.encode(&signing_secret).unwrap();

        let token = encoded.encoded().unwrap().to_string();
        tracing::debug!(token = ?token, "Encoded JWT");

        AccessToken(token)
    }

    #[test]
    fn authorize_valid_token() {
        let now = Utc::now().round_subsecs(0);

        let token = build_token(
            Some("userId"),
            Some("tag:worlds,2021:authorization"),
            Some("tag:worlds,2021:authorization"),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
            "secret",
        );

        let sut = AuthorizeSecurityContextUseCase::new("secret");

        let result = sut.authorize(token);

        let_assert!(Ok(token) = result);
        check!(token.principal == Principal::User("userId".to_owned()));
        check!(token.issued == now - Duration::days(5));
        check!(token.expires == now + Duration::days(5));
    }

    #[test_case(build_token(Some("userId"), Some("tag:worlds,2021:authorization"), Some("tag:worlds,2021:authorization"), Some(Utc::now() - Duration::days(5)), Some(Utc::now() - Duration::days(2)), "secret") ; "Expired")]
    #[test_case(build_token(Some("userId"), Some("tag:worlds,2021:authorization"), Some("tag:worlds,2021:authorization"), Some(Utc::now() + Duration::days(2)), Some(Utc::now() + Duration::days(5)), "secret") ; "Not Issued Yet")]
    #[test_case(build_token(Some("userId"), Some("wrong"), Some("tag:worlds,2021:authorization"), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "secret") ; "Wrong Issuer")]
    #[test_case(build_token(Some("userId"), Some("tag:worlds,2021:authorization"), Some("wrong"), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "secret") ; "Wrong Audience")]
    #[test_case(build_token(None, Some("tag:worlds,2021:authorization"), Some("tag:worlds,2021:authorization"), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "secret") ; "No Subject")]
    #[test_case(build_token(Some("userId"), Some("tag:worlds,2021:authorization"), Some("tag:worlds,2021:authorization"), None, Some(Utc::now() + Duration::days(5)), "secret") ; "No Issued Time")]
    #[test_case(build_token(Some("userId"), Some("tag:worlds,2021:authorization"), Some("tag:worlds,2021:authorization"), Some(Utc::now() - Duration::days(5)), None, "secret") ; "No Expiry Time")]
    #[test_case(build_token(Some("userId"), Some("tag:worlds,2021:authorization"), Some("tag:worlds,2021:authorization"), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "wrong") ; "Wrong Secret")]
    fn authorize_invalid_token(token: AccessToken) {
        let sut = AuthorizeSecurityContextUseCase::new("secret");

        let result = sut.authorize(token);

        let_assert!(Err(err) = result);
        check!(err == AuthorizeSecurityContextError::InvalidToken);
    }
}
