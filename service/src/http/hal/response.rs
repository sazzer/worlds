use super::HalDocument;
use crate::http::SimpleRespondable;

/// Respondable instance for HAL documents.
pub type HalRespondable = SimpleRespondable<HalDocument>;
