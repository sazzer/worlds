use biscuit::jwa::SignatureAlgorithm;

pub const ISSUER: &str = "tag:worlds,2021:authorization/issuer";
pub const AUDIENCE: &str = "tag:worlds,2021:authorization/audience";
pub const ALGORITHM: SignatureAlgorithm = SignatureAlgorithm::HS256;
