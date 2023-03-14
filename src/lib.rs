#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate core;

#[cfg(feature = "base64")]
mod base64;
mod config;
mod display;
mod eq;
mod error;
#[cfg(feature = "openapi")]
mod openapi;
mod parse;
mod secret;
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod tests;

use argon2::{password_hash::PasswordHashString, Argon2};
#[cfg(test)]
use rstest_reuse;

pub use config::{Algorithm, Params, SecretConfig, Version};
pub use error::SecretError;

pub type SecretResult<T> = Result<T, SecretError>;

#[derive(Clone)]
pub struct Secret {
    hasher: Argon2<'static>,
    secret: PasswordHashString,
}
