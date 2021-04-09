use actix_http::http::{HeaderMap, StatusCode};
use serde::Serialize;

/// Trait that anything able to represent a response can implement.
pub trait Respondable {
    type Body: Serialize;

    /// Generate the status code for the response
    ///
    /// # Returns
    /// The status code to send back to the client
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    /// Generate any headers for the response
    ///
    /// # Returns
    /// The headers to send back to the client
    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }

    /// Retrieve the body of the response
    ///
    /// # Returns
    /// The body to send back to the client
    fn body(self) -> Self::Body;
}

impl<T> Respondable for T
where
    T: Serialize,
{
    type Body = T;

    fn body(self) -> Self::Body {
        self
    }
}
