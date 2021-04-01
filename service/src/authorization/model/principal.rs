/// Enumeration of possible Principals for an authorized request
#[derive(Debug, PartialEq)]
pub enum Principal {
    /// The principal is a specific user.
    User(String),

    /// Additional unused principal type to force code to work correctly.
    #[allow(dead_code)]
    Unknown,
}
