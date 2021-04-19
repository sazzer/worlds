use serde::Serialize;

/// An access token, representing a signed security context.
#[derive(Debug, Serialize)]
pub struct AccessToken(pub String);
