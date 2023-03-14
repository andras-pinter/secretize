pub use argon2::{Algorithm, Params, Version};

#[derive(Debug, Default)]
pub struct SecretConfig {
    pub algorithm: Algorithm,
    pub version: Version,
    pub params: Params,
}
