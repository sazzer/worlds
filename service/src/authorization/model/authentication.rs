use std::{pin::Pin, sync::Arc};

use actix_http::{http::header, Payload};
use actix_web::{web::Data, FromRequest, HttpRequest};
use futures::Future;

use super::{Principal, SecurityContext};
use crate::{
    authorization::{service::AuthorizationService, AccessToken},
    http::problem::{Problem, UNAUTHORIZED},
};

/// Enumeration of possible authentication states.
#[derive(Debug)]
pub enum Authentication {
    /// Indication that a request is authenticated.
    Authenticated(SecurityContext),
    /// Indication that a request is unauthenticated.
    Unauthenticated,
}

impl Authentication {
    /// Determine if the request is authenticated or not.
    pub fn is_authenticated(&self) -> bool {
        !matches!(self, Authentication::Unauthenticated)
    }

    /// Get the security context that this authentication represents, if there is one.
    pub fn security_context(&self) -> Option<&SecurityContext> {
        match self {
            Authentication::Authenticated(sc) => Some(sc),
            Authentication::Unauthenticated => None,
        }
    }

    /// Get the authenticated principal that this authentication represents, if there is one.
    pub fn principal(&self) -> Option<&Principal> {
        self.security_context().map(|sc| &sc.principal)
    }
}

impl FromRequest for Authentication {
    type Config = ();
    type Error = Problem;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let authorization = req.headers().get(header::AUTHORIZATION).cloned();
        tracing::debug!("Processing authorization header: {:?}", authorization);

        let authorizer: &Data<Arc<AuthorizationService>> = req.app_data().unwrap();
        let authorizer = authorizer.get_ref().clone();

        Box::pin(async move {
            if let Some(authorization) = authorization {
                let token = authorization.to_str().map(|h| AccessToken(h.to_owned())).map_err(|e| {
                    tracing::warn!(e = ?e, authorization = ?authorization, "Failed to transform authorization header to string");
                    Problem::from(UNAUTHORIZED)
                })?;

                authorizer
                    .authorize(&token)
                    .map_err(|e| {
                        tracing::warn!(e = ?e, authorization = ?authorization, "Failed to authorize access token");
                        Problem::from(UNAUTHORIZED)
                    })
                    .map(Authentication::Authenticated)
            } else {
                Ok(Authentication::Unauthenticated)
            }
        })
    }
}

impl FromRequest for SecurityContext {
    type Config = ();
    type Error = Problem;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let authentication = Authentication::from_request(req, payload);

        Box::pin(async move {
            let authentication = authentication.await?;

            match authentication {
                Authentication::Authenticated(sc) => Ok(sc),
                Authentication::Unauthenticated => Err(Problem::from(UNAUTHORIZED)),
            }
        })
    }
}

impl FromRequest for Principal {
    type Config = ();
    type Error = Problem;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let authentication = Authentication::from_request(req, payload);

        Box::pin(async move {
            let authentication = authentication.await?;

            match authentication {
                Authentication::Authenticated(sc) => Ok(sc.principal),
                Authentication::Unauthenticated => Err(Problem::from(UNAUTHORIZED)),
            }
        })
    }
}
