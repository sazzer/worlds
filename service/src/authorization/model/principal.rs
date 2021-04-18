/// An authenticated principal.
#[derive(Debug, PartialEq)]
pub enum Principal {
    /// An authenticated user principal.
    User(String),
}
