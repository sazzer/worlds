use actix_web::web::{get, resource, ServiceConfig};

mod get;

/// Configure the HTTP routes for the home document.
///
/// # Parameters
/// - `config` - The HTTP Server configuration to register the routes with.
pub fn configure_routes(config: &mut ServiceConfig) {
    config.service(resource("/").route(get().to(get::handle)));
}
