#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, dead_code)]

mod authentication;
mod authorization;
mod database;
mod http;
mod model;
mod server;
mod service;
mod settings;
#[cfg(test)]
mod tests;
mod users;

use config::{Config, Environment};
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, Registry};

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    env_logger::init();

    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .install_simple()
        .unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let settings = load_settings();

    let service = service::Service::new(settings).await;
    service.start().await;
}

/// Load the application settings from the environment.
///
/// # Returns
/// The loaded settings.
fn load_settings() -> settings::Settings {
    let mut s = Config::new();
    s.set_default("port", 8000).expect("Failed to set default value for 'port'");

    s.merge(Environment::default()).expect("Failed to load environment properties");

    s.try_into().expect("Failed to build settings from config")
}
