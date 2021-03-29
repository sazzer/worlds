use serde::Deserialize;

/// The actual settings for the service.
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
}
