use super::AuthorizationService;
use crate::authorization::{AccessToken, Principal, SecurityContext};
use biscuit::{jwa::SignatureAlgorithm, jws::Compact, ClaimsSet, Validation, ValidationOptions};
use std::ops::Deref;

const ISSUER: &str = "tag:worlds,2021:authorization/issuer";
const AUDIENCE: &str = "tag:worlds,2021:authorization/audience";

/// Errors from authorizing an access token.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthorizeError {
    #[error("The access token was invalid")]
    InvalidToken,
}

impl AuthorizationService {
    /// Authorize an access token, producing the Security Context that it represents.
    ///
    /// # Parameters
    /// - `access_token` - The access token to authorize
    ///
    /// # Returns
    /// The security context, or an error if it can't be parsed.
    pub fn authorize(&self, access_token: &AccessToken) -> Result<SecurityContext, AuthorizeError> {
        let encoded = Compact::<ClaimsSet<()>, ()>::new_encoded(&access_token.0);
        let decoded = encoded
            .decode(&self.secret, SignatureAlgorithm::HS256)
            .map_err(|e| {
                tracing::warn!(e = ?e, access_token = ?access_token, "Failed to decode access token");
                AuthorizeError::InvalidToken
            })?;

        decoded
            .validate(ValidationOptions {
                issuer: Validation::Validate(ISSUER.to_owned()),
                audience: Validation::Validate(AUDIENCE.to_owned()),
                ..ValidationOptions::default()
            })
            .map_err(|e| {
                tracing::warn!(e = ?e, token = ?decoded, "Token validation failed");
                AuthorizeError::InvalidToken
            })?;

        let payload = decoded
            .payload()
            .map_err(|_| AuthorizeError::InvalidToken)?;

        let sub = payload.registered.subject.clone().ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "sub", "Missing field");
            AuthorizeError::InvalidToken
        })?;
        let iat = payload.registered.issued_at.ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "iat", "Missing field");
            AuthorizeError::InvalidToken
        })?;
        let exp = payload.registered.expiry.ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "exp", "Missing field");
            AuthorizeError::InvalidToken
        })?;

        Ok(SecurityContext {
            principal: Principal::User(sub),
            issued: *iat.deref(),
            expires: *exp.deref(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use biscuit::{
        jws::{RegisteredHeader, Secret},
        RegisteredClaims, SingleOrMultiple,
    };
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
                ..RegisteredHeader::default()
            }
            .into(),
            ClaimsSet::<()> {
                registered: RegisteredClaims {
                    issuer: iss.map(|s| s.parse().unwrap()),
                    subject: sub.map(|s| s.parse().unwrap()),
                    audience: aud.map(|s| SingleOrMultiple::Single(s.parse().unwrap())),
                    issued_at: iat.map(|t| t.into()),
                    expiry: exp.map(|t| t.into()),
                    ..RegisteredClaims::default()
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
            Some(ISSUER),
            Some(AUDIENCE),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
            "secret",
        );

        let sut = AuthorizationService::new("secret");

        let result = sut.authorize(&token);

        let_assert!(Ok(token) = result);
        check!(token.principal == Principal::User("userId".to_owned()));
        check!(token.issued == now - Duration::days(5));
        check!(token.expires == now + Duration::days(5));
    }

    #[test_case(&build_token(Some("userId"), Some(ISSUER), Some(AUDIENCE), Some(Utc::now() - Duration::days(5)), Some(Utc::now() - Duration::days(2)), "secret") ; "Expired")]
    #[test_case(&build_token(Some("userId"), Some(ISSUER), Some(AUDIENCE), Some(Utc::now() + Duration::days(2)), Some(Utc::now() + Duration::days(5)), "secret") ; "Not Issued Yet")]
    #[test_case(&build_token(Some("userId"), Some("wrong"), Some(AUDIENCE), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "secret") ; "Wrong Issuer")]
    #[test_case(&build_token(Some("userId"), Some(ISSUER), Some("wrong"), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "secret") ; "Wrong Audience")]
    #[test_case(&build_token(None, Some(ISSUER), Some(AUDIENCE), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "secret") ; "No Subject")]
    #[test_case(&build_token(Some("userId"), Some(ISSUER), Some(AUDIENCE), None, Some(Utc::now() + Duration::days(5)), "secret") ; "No Issued Time")]
    #[test_case(&build_token(Some("userId"), Some(ISSUER), Some(AUDIENCE), Some(Utc::now() - Duration::days(5)), None, "secret") ; "No Expiry Time")]
    #[test_case(&build_token(Some("userId"), Some(ISSUER), Some(AUDIENCE), Some(Utc::now() - Duration::days(5)), Some(Utc::now() + Duration::days(5)), "wrong") ; "Wrong Secret")]
    fn authorize_invalid_token(token: &AccessToken) {
        let sut = AuthorizationService::new("secret");

        let result = sut.authorize(&token);

        let_assert!(Err(err) = result);
        check!(err == AuthorizeError::InvalidToken);
    }
}
