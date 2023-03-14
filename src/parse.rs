use std::str::FromStr;

use crate::{Secret, SecretError};

#[cfg(not(feature = "base64"))]
impl FromStr for Secret {
    type Err = SecretError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Secret::load(s)
    }
}

#[cfg(feature = "base64")]
impl FromStr for Secret {
    type Err = SecretError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::load_from_base64(s).or(Self::load(s))
    }
}
