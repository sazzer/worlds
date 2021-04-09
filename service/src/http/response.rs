mod respondable;
mod simple;

use actix_web::{HttpRequest, HttpResponse, Responder};
pub use respondable::*;
use serde::Serialize;
pub use simple::*;

/// Wrapper for any HTTP Response, implementing the standard requirements.
///
/// # Types
/// - `R` - The exact type of `Respondable` to wrap.
pub struct Response<R>(pub R)
where
    R: Respondable,
    R::Body: Serialize;

impl<R> From<R> for Response<R>
where
    R: Respondable,
    R::Body: Serialize,
{
    fn from(respondable: R) -> Self {
        Self(respondable)
    }
}

impl<R> Responder for Response<R>
where
    R: Respondable,
    R::Body: Serialize,
{
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let mut response = HttpResponse::build(self.0.status_code());

        for (key, value) in self.0.headers().iter() {
            response.insert_header((key, value.clone()));
        }

        response.json(self.0.body())
    }
}
