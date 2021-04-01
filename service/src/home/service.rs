use crate::{authorization::Authentication, http::hal::Link};
use std::sync::Arc;

/// Trait for all components that can contribute links to the home document.
pub trait LinkContributor: Send + Sync {
    /// Generate the links for this component.
    fn generate_links(&self, authentication: &Authentication) -> Vec<(String, Link)>;
}

/// Use Case for generating the entire set of links for the home document.
pub struct HomeLinksUseCase {
    pub(super) contributors: Vec<Arc<dyn LinkContributor>>,
}

impl HomeLinksUseCase {
    /// Generate the links for this component.
    pub fn generate_links(&self, authentication: &Authentication) -> Vec<(String, Link)> {
        let mut result = vec![];

        for c in &self.contributors {
            let mut links = c.generate_links(authentication);
            result.append(&mut links);
        }

        result
    }
}

impl LinkContributor for Vec<(String, Link)> {
    fn generate_links(&self, _: &Authentication) -> Vec<(String, Link)> {
        self.clone()
    }
}
