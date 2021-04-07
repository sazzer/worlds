use super::Server;

/// Builder for the HTTP Server component.
#[derive(Default)]
pub struct Builder {}

/// The HTTP Server component.
pub struct Component {
    pub server: Server,
}

impl Builder {
    /// Build the HTTP Server component.
    pub fn build(self, port: u16) -> Component {
        Component {
            server: Server::new(port),
        }
    }
}
