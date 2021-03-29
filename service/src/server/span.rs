use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Middleware for applying a tracing `Span` around the entire HTTP request, and tracking certain details on it.
pub struct Span;

impl<S, B> Transform<S> for Span
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = Middleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(Middleware { service })
    }
}

/// Actual middleware implementation.
pub struct Middleware<S> {
    service: S,
}

impl<S, B> Service for Middleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let span = tracing::trace_span!(
            "Request",
            http.method = req.method().as_str(),
            http.path = req.path(),
            http.status_code = tracing::field::Empty
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let span = span;
            let _enter = span.enter();

            let response = fut.await?;

            span.record("http.status_code", &response.status().as_u16());

            Ok(response)
        })
    }
}
