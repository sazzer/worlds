use std::pin::Pin;

use super::SecurityContext;
use crate::http::problem::{Problem, UNAUTHORIZED};
use actix_http::{http::header, Payload};
use actix_web::{FromRequest, HttpRequest};
use futures::Future;

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
            if let Some(_) = authorization {
                Err(Problem::from(UNAUTHORIZED))
            } else {
                Ok(Authentication::Unauthenticated)
            }
        })
    }
}
