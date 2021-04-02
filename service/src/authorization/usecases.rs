use super::{AccessToken, Principal, SecurityContext};
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{Compact, RegisteredHeader, Secret},
    ClaimsSet, RegisteredClaims, SingleOrMultiple, Validation, ValidationOptions,
};
use chrono::{Duration, SubsecRound, Utc};
use std::ops::Deref;

const ISSUER: &str = "tag:worlds,2021:authorization";
const AUDIENCE: &str = "tag:worlds,2021:authorization";

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
    pub fn new(secret: &str) -> Self {
        let signing_secret = Secret::Bytes(secret.to_owned().into_bytes());

        Self { secret: signing_secret }
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
                issuer: Validation::Validate(ISSUER.to_owned()),
                audience: Validation::Validate(AUDIENCE.to_owned()),
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

/// Use case for generating a security context for a principal.
pub struct GenerateSecurityContextUseCase {
    secret: Secret,
    duration: Duration,
}

impl GenerateSecurityContextUseCase {
    /// Create a new instance of the use case.
    pub fn new(secret: &str, duration: Duration) -> Self {
        let signing_secret = Secret::Bytes(secret.to_owned().into_bytes());

        Self {
            secret: signing_secret,
            duration,
        }
    }

    /// Generate a security context and access token for the provided principal.
    ///
    /// # Parameters
    /// - `principal` - The principal to generate the security context for
    ///
    /// # Returns
    /// A security context and associated access token
    pub fn generate(&self, principal: Principal) -> (SecurityContext, AccessToken) {
        // Issue the token a second ago to ensure that the time is in the past.
        let issued = Utc::now().round_subsecs(0) - Duration::seconds(1);
        let expires = issued + self.duration;

        let decoded = Compact::new_decoded(
            RegisteredHeader {
                algorithm: SignatureAlgorithm::HS256,
                ..Default::default()
            }
            .into(),
            ClaimsSet::<()> {
                registered: RegisteredClaims {
                    issuer: Some(ISSUER.to_owned()),
                    audience: Some(SingleOrMultiple::Single(AUDIENCE.to_owned())),
                    subject: match &principal {
                        Principal::User(user_id) => Some(user_id.clone()),
                        Principal::Unknown => None,
                    },
                    issued_at: Some(issued.into()),
                    expiry: Some(expires.into()),
                    ..Default::default()
                },
                private: (),
            },
        );

        let encoded = decoded.encode(&self.secret).unwrap();

        let token = encoded.encoded().unwrap().to_string();
        tracing::debug!(token = ?token, "Encoded JWT");

        (SecurityContext { principal, expires, issued }, AccessToken(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use chrono::DateTime;
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

    #[test]
    fn generate_token() {
        let _ = env_logger::try_init();

        let authorize_sut = AuthorizeSecurityContextUseCase::new("secret");
        let generate_sut = GenerateSecurityContextUseCase::new("secret", Duration::days(5));

        let principal = Principal::User("myUserId".to_owned());

        let (sc, token) = generate_sut.generate(principal);

        check!(sc.principal == Principal::User("myUserId".to_owned()));
        check!(sc.issued + Duration::days(5) == sc.expires);

        let authorized = authorize_sut.authorize(token);
        let_assert!(Ok(authorized) = authorized);
        check!(authorized == sc);
    }
}
