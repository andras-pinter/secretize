#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("Failed to hash secret")]
    Hashing(argon2::password_hash::Error),
    #[error("Failed to parse secret hash")]
    HashParsing(argon2::password_hash::Error),
    #[error("Invalid hasher algorithm")]
    InvalidAlgorithm(argon2::password_hash::Error),
    #[error("Invalid hasher version")]
    InvalidVersion(argon2::Error),
    #[error("Invalid hasher parameters")]
    InvalidParams(argon2::password_hash::Error),
    #[cfg(feature = "base64")]
    #[error("Invalid Base64")]
    InvalidBase64(base64::DecodeError),
    #[cfg(feature = "base64")]
    #[error("Invalid UTF-8")]
    InvalidUtf8(std::string::FromUtf8Error),
}
