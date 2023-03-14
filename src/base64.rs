pub use base64::Engine;

use crate::{Secret, SecretError, SecretResult};

#[allow(non_upper_case_globals)]
pub const b64Engine: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::STANDARD,
    base64::engine::general_purpose::GeneralPurposeConfig::new(),
);

impl Secret {
    pub fn to_base64(&self) -> String {
        b64Engine.encode(self.to_string())
    }

    pub fn load_from_base64<S: AsRef<[u8]>>(s: S) -> SecretResult<Self> {
        let encoded = b64Engine.decode(s).map_err(SecretError::InvalidBase64)?;
        let raw = String::from_utf8(encoded).map_err(SecretError::InvalidUtf8)?;
        Self::load(raw)
    }
}
