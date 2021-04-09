use super::Respondable;
use actix_http::http::{header::Header, HeaderMap, StatusCode};
use serde::Serialize;

/// Simple implementation of the Respondable trait.
///
/// # Types
/// - `T` - The type to use for the response body.
pub struct SimpleRespondable<T>
where
    T: Serialize,
{
    status_code: StatusCode,
    headers: HeaderMap,
    body: T,
}

impl<T> SimpleRespondable<T>
where
    T: Serialize,
{
    /// Create a new instance of the `SimpleRespondable` struct wrapping the provided body.
    ///
    /// # Parameters
    /// - `body` - The body to send back to the client.
    pub fn new(body: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
            body,
        }
    }

    /// Specify the status code to use.
    ///
    /// # Parameters
    /// - `status_code` - The status code to use
    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    /// Specify a header to include in the response.
    ///
    /// # Parameters
    /// - `header` - The header to add to the response.
    pub fn with_header<H>(mut self, header: H) -> Self
    where
        H: Header,
    {
        let name = H::name();
        match header.try_into_value() {
            Ok(value) => {
                self.headers.append(name, value);
            }
            Err(_) => {
                tracing::error!(name = ?name, "Failed to process header");
            }
        };

        self
    }
}

impl<T> Respondable for SimpleRespondable<T>
where
    T: Serialize,
{
    type Body = T;

    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    fn body(self) -> Self::Body {
        self.body
    }
}
