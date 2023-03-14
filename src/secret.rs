use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use zeroize::Zeroize;

use crate::{Secret, SecretConfig, SecretError, SecretResult};

impl Secret {
    pub fn new<S: AsRef<[u8]>>(secret: S) -> SecretResult<Self> {
        Self::new_with_config(secret, SecretConfig::default())
    }

    pub fn wrap<S: AsRef<[u8]> + Zeroize>(secret: S) -> SecretResult<Self> {
        Self::wrap_with_config(secret, SecretConfig::default())
    }

    pub fn new_with_config<S: AsRef<[u8]>>(secret: S, config: SecretConfig) -> SecretResult<Self> {
        Self::hash_secret(secret.as_ref(), config)
    }

    pub fn wrap_with_config<S: AsRef<[u8]> + Zeroize>(
        mut secret: S,
        config: SecretConfig,
    ) -> SecretResult<Self> {
        match Self::hash_secret(secret.as_ref(), config) {
            Ok(sec) => {
                secret.zeroize();
                Ok(sec)
            }
            Err(err) => Err(err),
        }
    }

    pub fn load<S: AsRef<str>>(secretized: S) -> SecretResult<Self> {
        let secret_hash =
            PasswordHash::new(secretized.as_ref()).map_err(SecretError::HashParsing)?;

        let algorithm = argon2::Algorithm::try_from(secret_hash.algorithm)
            .map_err(SecretError::InvalidAlgorithm)?;
        let version = match secret_hash.version {
            Some(ver) => argon2::Version::try_from(ver).map_err(SecretError::InvalidVersion)?,
            None => argon2::Version::default(),
        };
        let params = argon2::Params::try_from(&secret_hash).map_err(SecretError::InvalidParams)?;

        Ok(Secret {
            hasher: Argon2::new(algorithm, version, params),
            secret: secret_hash.serialize(),
        })
    }

    pub fn verify<S: AsRef<[u8]>>(&self, secret: S) -> bool {
        self.hasher
            .verify_password(secret.as_ref(), &self.secret.password_hash())
            .is_ok()
    }

    fn hash_secret(secret: &[u8], config: SecretConfig) -> SecretResult<Self> {
        let hasher = Argon2::new(config.algorithm, config.version, config.params);
        let salt = SaltString::generate(&mut OsRng);
        let secret_hash = hasher
            .hash_password(secret.as_ref(), &salt)
            .map_err(SecretError::Hashing)?
            .serialize();

        Ok(Secret {
            hasher,
            secret: secret_hash,
        })
    }
}
