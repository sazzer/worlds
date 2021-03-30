use super::{AccessToken, SecurityContext};
use crate::http::problem::{Problem, UNAUTHORIZED};
use actix_http::{http::header, Payload};
use actix_web::{FromRequest, HttpRequest};
use futures::Future;
use std::pin::Pin;

/// Authentication details for a request
#[derive(Debug)]
pub enum Authentication {
    /// The request is unauthenticated.
    Unauthenticated,
    /// The request is authenticated.
    Authenticated(SecurityContext),
}

impl Authentication {
    pub fn is_authenticated(&self) -> bool {
        match self {
            &Authentication::Unauthenticated => false,
            _ => true,
        }
    }

    pub fn security_context(&self) -> Option<&SecurityContext> {
        match self {
            Authentication::Authenticated(sc) => Some(sc),
            Authentication::Unauthenticated => None,
        }
    }
}

impl FromRequest for Authentication {
    type Error = Problem;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    #[tracing::instrument(skip(req))]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let authorization = req.headers().get(header::AUTHORIZATION).cloned();
        tracing::debug!("Processing authorization header: {:?}", authorization);

        Box::pin(async move {
            if let Some(authorization) = authorization {
                let _token: AccessToken = authorization
                    .to_str()
                    .map_err(|e| {
                        tracing::warn!(e = ?e, authorization = ?authorization, "Failed to transform authorization header to string");
                        Problem::from(UNAUTHORIZED)
                    })?
                    .parse()
                    .map_err(|e| {
                        tracing::warn!(e = ?e, authorization = ?authorization, "Failed to parse access token");
                        Problem::from(UNAUTHORIZED)
                    })?;

                Err(Problem::from(UNAUTHORIZED))
            } else {
                Ok(Authentication::Unauthenticated)
            }
        })
    }
}
