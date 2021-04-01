use super::home::HomeLinks;
use crate::home::LinkContributor;
use std::sync::Arc;

/// Component for working with users.
pub struct Component {
    pub home_links: Arc<dyn LinkContributor>,
}

/// Create a new instance of the users component.
pub fn new() -> Arc<Component> {
    let home_links = HomeLinks {};

    Arc::new(Component {
        home_links: Arc::new(home_links),
    })
}
