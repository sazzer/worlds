use crate::home::HomeLinksUseCase;
use crate::http::{hal::HalDocument, hal::HalRespondable, Response};
use actix_http::http::{
    header::{CacheControl, CacheDirective},
    StatusCode,
};
use actix_web::web::Data;
use serde::Serialize;
use std::sync::Arc;

/// The actual home document contents.
#[derive(Serialize)]
pub struct HomeDocument {
    pub name: &'static str,
    pub version: &'static str,
}

/// Generate the home document
pub async fn handle(home_links: Data<Arc<HomeLinksUseCase>>) -> Response<HalRespondable> {
    let mut hal_document = HalDocument::new(HomeDocument {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    });

    let links = home_links.generate_links().await;
    for (name, link) in links {
        hal_document = hal_document.with_link(name, link);
    }

    HalRespondable::from(hal_document)
        .with_status_code(StatusCode::OK)
        .with_header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(3600)]))
        .into()
}
