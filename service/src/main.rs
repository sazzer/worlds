#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod service;

use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

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

    let service = service::Service::new().await;
    service.start().await;
}
