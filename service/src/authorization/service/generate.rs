use super::constants::{ALGORITHM, AUDIENCE, ISSUER};
use super::AuthorizationService;
use crate::authorization::{AccessToken, Principal, SecurityContext};
use biscuit::{
    jws::{Compact, RegisteredHeader},
    ClaimsSet, RegisteredClaims, SingleOrMultiple,
};
use chrono::{Duration, SubsecRound, Utc};

impl AuthorizationService {
    pub fn generate_security_context(
        &self,
        principal: Principal,
    ) -> (SecurityContext, AccessToken) {
        let issued = Utc::now().round_subsecs(0) - Duration::seconds(1); // Needs to be in the past, so deduct one second from it.
        let expires = issued + Duration::days(30);
        let security_context = SecurityContext {
            issued,
            expires,
            principal,
        };

        let decoded = Compact::new_decoded(
            RegisteredHeader {
                algorithm: ALGORITHM,
                ..RegisteredHeader::default()
            }
            .into(),
            ClaimsSet::<()> {
                registered: RegisteredClaims {
                    issuer: Some(ISSUER.to_owned()),
                    subject: match &security_context.principal {
                        Principal::User(user_id) => Some(user_id.clone()),
                    },
                    audience: Some(SingleOrMultiple::Single(AUDIENCE.to_owned())),
                    issued_at: Some(security_context.issued.clone().into()),
                    expiry: Some(security_context.expires.clone().into()),
                    ..RegisteredClaims::default()
                },
                private: (),
            },
        );

        let encoded = decoded.encode(&self.secret).unwrap();
        let token = encoded.encoded().unwrap().to_string();

        (security_context, AccessToken(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};

    #[test]
    fn generate_security_context() {
        let sut = AuthorizationService::new("secret");

        let (security_context, access_token) =
            sut.generate_security_context(Principal::User("user_id".to_owned()));

        check!(security_context.issued + Duration::days(30) == security_context.expires);
        check!(security_context.principal == Principal::User("user_id".to_owned()));

        let authorized = sut.authorize(&access_token);
        let_assert!(Ok(authorized_security_token) = authorized);
        check!(authorized_security_token.issued == security_context.issued);
        check!(authorized_security_token.expires == security_context.expires);
        check!(authorized_security_token.principal == security_context.principal);
    }
}
