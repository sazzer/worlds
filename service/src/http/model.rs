use super::response::{Response, SimpleRespondable};
use crate::model::Resource;
use actix_http::http::StatusCode;
use actix_web::http::header::{CacheControl, CacheDirective, ETag, EntityTag};
use serde::Serialize;

/// Trait that can be implemented by resources to provide response details to use.
pub trait ResourceResponse {
    /// Produce the status code to use for the response.
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    /// Produce the Etag to use for the response.
    fn etag(&self) -> Option<EntityTag> {
        None
    }

    /// Produce the cache directives to use for the response.
    fn cache_control(&self) -> Option<Vec<CacheDirective>> {
        None
    }
}

impl<O, I, D> From<Resource<I, D>> for SimpleRespondable<O>
where
    O: Serialize,
    Resource<I, D>: Into<O> + ResourceResponse,
{
    fn from(resource: Resource<I, D>) -> Self {
        let status_code = resource.status_code();
        let etag = resource
            .etag()
            .unwrap_or_else(|| EntityTag::strong(resource.identity.version.to_string()));
        let cache_control = resource.cache_control();

        let mut result = Self::new(resource.into())
            .with_status_code(status_code)
            .with_header(ETag(etag));

        if let Some(cache_control) = cache_control {
            result = result.with_header(CacheControl(cache_control));
        }

        result
    }
}

impl<O, I, D> From<Resource<I, D>> for Response<SimpleRespondable<O>>
where
    O: Serialize,
    Resource<I, D>: Into<O> + ResourceResponse,
{
    fn from(resource: Resource<I, D>) -> Self {
        SimpleRespondable::<O>::from(resource).into()
    }
}
