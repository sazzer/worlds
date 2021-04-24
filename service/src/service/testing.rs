use actix_http::{http::header::IntoHeaderPair, Request};
use actix_web::App;

use super::Service;
use crate::authorization::Principal;

impl Service {
    /// Inject a request into the server. Only used for testing.
    ///
    /// # Parameters
    /// - `req` - The request to inject
    ///
    /// # Returns
    /// The response from injecting the request.
    pub async fn inject(&self, req: Request) -> TestResponse {
        let mut app = App::new();
        for c in &self.server.routes {
            app = app.configure(move |server_config| {
                c.configure_routes(server_config);
            });
        }

        let test_service = actix_web::test::init_service(app).await;
        let response = actix_web::test::call_service(&test_service, req).await;

        let status = response.status();
        let headers = response.headers().clone();
        let body = actix_web::test::read_body(response).await;

        TestResponse { status, headers, body }
    }

    pub fn authorize(&self, user_id: &str) -> impl IntoHeaderPair {
        let (_, token) = self
            .authorization_service
            .generate_security_context(Principal::User(user_id.into()));

        ("Authorization", token.0)
    }
}

/// Representation of the response to injecting a test request
pub struct TestResponse {
    /// The status code
    pub status:  actix_http::http::StatusCode,
    /// The set of headers
    pub headers: actix_http::http::HeaderMap,
    /// The response body
    pub body:    actix_web::web::Bytes,
}

impl TestResponse {
    /// Get the value of the header with the given name
    ///
    /// # Parameters
    /// - `name` - The name of the header
    ///
    /// # Returns
    /// The header, if present. `None` if it wasn't present.
    #[allow(dead_code)]
    pub fn header<S>(&self, name: S) -> Option<&actix_http::http::HeaderValue>
    where
        S: Into<String>,
    {
        self.headers.get(name.into())
    }

    /// Convert the response body to JSON
    ///
    /// # Returns
    /// The body of the response, converted to a Serde JSON object
    ///
    /// # Errors
    /// Any errors from deserializing the response
    #[allow(dead_code)]
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::error::Error> {
        serde_json::from_slice(&self.body)
    }
}
