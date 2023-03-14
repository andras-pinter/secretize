use crate::{Secret, SecretResult};

trait SerDeser {
    fn ser(&self) -> String;
    fn deser(s: String) -> SecretResult<Self>
    where
        Self: Sized;
}

#[cfg(not(feature = "base64"))]
impl SerDeser for Secret {
    fn ser(&self) -> String {
        self.to_string()
    }

    fn deser(s: String) -> SecretResult<Self> {
        Self::load(s)
    }
}

#[cfg(feature = "base64")]
impl SerDeser for Secret {
    fn ser(&self) -> String {
        self.to_base64()
    }

    fn deser(s: String) -> SecretResult<Self> {
        Self::load_from_base64(s)
    }
}

impl serde::Serialize for Secret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.ser().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Secret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deser(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
