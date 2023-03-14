use parking_lot::RwLock;
use rstest_reuse::{self, *};
use std::sync::Arc;

const TEST_SECRET: &str = "my-secret";

#[template]
#[rstest::rstest]
#[case::string(TEST_SECRET.to_string())]
#[case::vector(TEST_SECRET.as_bytes().to_vec())]
#[case::boxed_bytes(TEST_SECRET.to_string().into_bytes())]
fn test_cases<S: AsRef<[u8]> + zeroize::Zeroize>(#[case] secret_input: S) {}

struct MemoryFootprintTestcase<S: AsRef<[u8]>> {
    inner: S,
    store: Arc<RwLock<Vec<u8>>>,
}

impl<S: AsRef<[u8]>> MemoryFootprintTestcase<S> {
    fn new(secret: S, store: Arc<RwLock<Vec<u8>>>) -> Self {
        *store.write() = secret.as_ref().to_vec();
        MemoryFootprintTestcase {
            inner: secret,
            store,
        }
    }
}

impl<S: AsRef<[u8]>> AsRef<[u8]> for MemoryFootprintTestcase<S> {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl<S: AsRef<[u8]> + zeroize::Zeroize> zeroize::Zeroize for MemoryFootprintTestcase<S> {
    fn zeroize(&mut self) {
        self.inner.zeroize();
        *self.store.write() = self.inner.as_ref().to_vec()
    }
}

mod base {
    use parking_lot::RwLock;
    use rstest_reuse::{self, *};
    use std::sync::Arc;
    use zeroize::Zeroize;

    use crate::tests::{test_cases, MemoryFootprintTestcase, TEST_SECRET};
    use crate::Secret;

    #[apply(test_cases)]
    #[case::string(TEST_SECRET)]
    #[case::string(TEST_SECRET.as_bytes())]
    fn test_secretize_new<S: AsRef<[u8]>>(secret_input: S) {
        let secretized = Secret::new(secret_input);
        assert!(secretized.is_ok());

        let secretized = secretized.unwrap();
        assert!(!secretized.to_string().contains(TEST_SECRET));
    }

    #[apply(test_cases)]
    fn test_secretize_wrap<S: AsRef<[u8]> + Zeroize>(secret_input: S) {
        let secretized = Secret::wrap(secret_input);
        assert!(secretized.is_ok());

        let secretized = secretized.unwrap();
        assert!(!secretized.to_string().contains(TEST_SECRET));
    }

    #[apply(test_cases)]
    fn test_secret_load<S: AsRef<[u8]> + Zeroize>(secret_input: S) {
        let secretized = Secret::wrap(secret_input)
            .expect("Invalid secret hash")
            .to_string();

        let secretized = Secret::load(&secretized);
        assert!(secretized.is_ok());
        let secretized = secretized.unwrap();
        assert!(!secretized.to_string().contains(TEST_SECRET));
    }

    #[apply(test_cases)]
    fn test_secret_verify<S: AsRef<[u8]> + Zeroize>(secret_input: S) {
        let secretized = Secret::wrap(secret_input).expect("Invalid secret hash");
        assert!(secretized.verify(TEST_SECRET));
        assert!(!secretized.verify("not-my-secret"));

        let secretized = Secret::load(&secretized.to_string()).expect("Invalid secret hash load");
        assert!(secretized.verify(TEST_SECRET));
        assert!(!secretized.verify("not-my-secret"));
    }

    #[apply(test_cases)]
    fn test_secretize_memory_footprint<S: AsRef<[u8]> + Zeroize>(secret_input: S) {
        let store = Arc::new(RwLock::default());
        let memory_footprint_testcase = MemoryFootprintTestcase::new(secret_input, store.clone());
        assert_eq!(String::from_utf8_lossy(&store.read()), TEST_SECRET);

        let secretized = Secret::wrap(memory_footprint_testcase);
        assert!(secretized.is_ok());
        assert!(store.read().is_empty())
    }

    #[apply(test_cases)]
    #[case::string(TEST_SECRET)]
    #[case::string(TEST_SECRET.as_bytes())]
    fn test_secretize_memory_footprint_for_new<S: AsRef<[u8]>>(secret_input: S) {
        let store = Arc::new(RwLock::default());
        let memory_footprint_testcase = MemoryFootprintTestcase::new(secret_input, store.clone());
        assert_eq!(String::from_utf8_lossy(&store.read()), TEST_SECRET);

        let secretized = Secret::new(memory_footprint_testcase);
        assert!(secretized.is_ok());
        assert!(!store.read().is_empty())
    }

    #[rstest::rstest]
    fn test_parse() {
        let secretized = "$argon2id$v=19$m=19456,t=2,p=1$EPyZixFuc12NtIBjEtnRaA$EVfkzdbkxEq5wvvajH66helPj12WjcVw4hcGHquNwSk".parse::<Secret>();
        assert!(secretized.is_ok());

        let secretized = secretized.unwrap();
        assert!(!secretized.to_string().contains(TEST_SECRET));
        assert!(secretized.verify(TEST_SECRET));
    }

    #[rstest::rstest]
    fn test_parse_and_from_str() {
        let loaded = Secret::load("$argon2id$v=19$m=19456,t=2,p=1$EPyZixFuc12NtIBjEtnRaA$EVfkzdbkxEq5wvvajH66helPj12WjcVw4hcGHquNwSk");
        let parsed = "$argon2id$v=19$m=19456,t=2,p=1$EPyZixFuc12NtIBjEtnRaA$EVfkzdbkxEq5wvvajH66helPj12WjcVw4hcGHquNwSk".parse::<Secret>();
        assert!(loaded.is_ok());
        assert!(parsed.is_ok());
        assert_eq!(loaded.unwrap().to_string(), parsed.unwrap().to_string(),)
    }
}

#[cfg(feature = "base64")]
mod test_base64 {
    use crate::tests::TEST_SECRET;
    use crate::Secret;

    #[rstest::rstest]
    fn test_parse_base() {
        let secretized = "JGFyZ29uMmlkJHY9MTkkbT0xOTQ1Nix0PTIscD0xJDliNGpYd2xFS1FsenNCVkRHd3JrMWckM0ZiMit5aEJTMU1FSm9BeitTVW5OVmcvMTlTdi8vMTdIUEY5YXVnMForWQ==".parse::<Secret>();
        assert!(secretized.is_ok());

        let secretized = secretized.unwrap();
        assert!(!secretized.to_string().contains(TEST_SECRET));
        assert!(secretized.verify(TEST_SECRET));
    }

    #[rstest::rstest]
    fn test_parse_and_from_base64() {
        let loaded = Secret::load_from_base64("JGFyZ29uMmlkJHY9MTkkbT0xOTQ1Nix0PTIscD0xJDliNGpYd2xFS1FsenNCVkRHd3JrMWckM0ZiMit5aEJTMU1FSm9BeitTVW5OVmcvMTlTdi8vMTdIUEY5YXVnMForWQ==");
        let parsed = "JGFyZ29uMmlkJHY9MTkkbT0xOTQ1Nix0PTIscD0xJDliNGpYd2xFS1FsenNCVkRHd3JrMWckM0ZiMit5aEJTMU1FSm9BeitTVW5OVmcvMTlTdi8vMTdIUEY5YXVnMForWQ==".parse::<Secret>();
        assert!(loaded.is_ok());
        assert!(parsed.is_ok());
        assert_eq!(loaded.unwrap().to_string(), parsed.unwrap().to_string(),)
    }
}

#[cfg(feature = "serde")]
mod test_serde {
    use crate::tests::TEST_SECRET;
    use crate::Secret;

    #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
    struct TestSecretWrapper {
        secret: Secret,
    }

    #[cfg(not(feature = "base64"))]
    #[rstest::rstest]
    fn test_serialize() {
        let secret = Secret::new(TEST_SECRET).expect("invalid secret");
        let serialized = serde_json::to_string(&secret);
        assert!(serialized.is_ok());
    }

    #[cfg(not(feature = "base64"))]
    #[rstest::rstest]
    fn test_deserialize() {
        let deserialized = serde_json::from_str::<Secret>("\"$argon2id$v=19$m=19456,t=2,p=1$EPyZixFuc12NtIBjEtnRaA$EVfkzdbkxEq5wvvajH66helPj12WjcVw4hcGHquNwSk\"");
        assert!(deserialized.is_ok());
    }

    #[cfg(feature = "base64")]
    #[rstest::rstest]
    fn test_serialize_base64() {
        use crate::base64::{b64Engine, Engine};

        let secret = Secret::new(TEST_SECRET).expect("invalid secret");
        let serialized = serde_json::to_string(&secret);
        assert!(serialized.is_ok());
        assert!(b64Engine
            .decode(serialized.unwrap().trim_matches('"'))
            .is_ok());
    }

    #[cfg(feature = "base64")]
    #[rstest::rstest]
    fn test_deserialize_base64() {
        let deserialized = serde_json::from_str::<Secret>("\"JGFyZ29uMmlkJHY9MTkkbT0xOTQ1Nix0PTIscD0xJDliNGpYd2xFS1FsenNCVkRHd3JrMWckM0ZiMit5aEJTMU1FSm9BeitTVW5OVmcvMTlTdi8vMTdIUEY5YXVnMForWQ==\"");
        assert!(deserialized.is_ok());
    }
}

#[cfg(feature = "openapi")]
mod test_openapi {
    use poem_openapi::registry::{MetaSchemaRef, Registry};
    use poem_openapi::types::{ParseFromJSON, ToJSON, Type};

    use crate::tests::TEST_SECRET;
    use crate::Secret;

    #[rstest::rstest]
    fn test_openapi() {
        let mut registry = Registry::new();
        let secret = Secret::new(TEST_SECRET).expect("invalid secret");
        assert_eq!(Secret::name(), "Secret");
        assert_eq!(
            Secret::schema_ref(),
            MetaSchemaRef::Reference("Secret".to_string())
        );
        assert!(secret.to_json().is_some());

        assert!(
            Secret::parse_from_json(Some(serde_json::json!({ "secret": TEST_SECRET }))).is_ok()
        );

        Secret::register(&mut registry);
        let schema = registry.create_fake_schema::<Secret>();
        assert!(!schema.is_empty());
        let schema_json = serde_json::to_string(&schema);
        assert!(schema_json.is_ok());
    }

    #[cfg(not(feature = "base64"))]
    #[rstest::rstest]
    fn test_openapi_json_representation() {
        let secret = Secret::new(TEST_SECRET).expect("invalid secret");
        assert!(secret.to_json().is_some());
        assert!(secret.to_json().unwrap().as_object().is_some());
        assert!(secret
            .to_json()
            .unwrap()
            .as_object()
            .unwrap()
            .get("secret")
            .is_some());
        assert!(secret
            .to_json()
            .unwrap()
            .as_object()
            .unwrap()
            .get("secret")
            .unwrap()
            .as_str()
            .is_some());

        let secret = secret
            .to_json()
            .unwrap()
            .as_object()
            .unwrap()
            .get("secret")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let secret = Secret::load(secret);
        assert!(secret.is_ok());
        let secret = secret.unwrap();
        assert!(secret.verify(TEST_SECRET));
        assert!(!secret.verify("not-my-secret"));
    }

    #[cfg(feature = "base64")]
    #[rstest::rstest]
    fn test_openapi_json_representation() {
        let secret = Secret::new(TEST_SECRET).expect("invalid secret");
        assert!(secret.to_json().is_some());
        assert!(secret.to_json().unwrap().as_object().is_some());
        assert!(secret
            .to_json()
            .unwrap()
            .as_object()
            .unwrap()
            .get("secret")
            .is_some());
        assert!(secret
            .to_json()
            .unwrap()
            .as_object()
            .unwrap()
            .get("secret")
            .unwrap()
            .as_str()
            .is_some());

        let secret = secret
            .to_json()
            .unwrap()
            .as_object()
            .unwrap()
            .get("secret")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let secret = Secret::load_from_base64(secret);
        assert!(secret.is_ok());
        let secret = secret.unwrap();
        assert!(secret.verify(TEST_SECRET));
        assert!(!secret.verify("not-my-secret"));
    }
}

#[cfg(feature = "eq")]
mod test_qe {
    use crate::tests::TEST_SECRET;
    use crate::Secret;

    #[rstest::rstest]
    fn test_secret_equality() {
        let secret = Secret::new(TEST_SECRET).expect("invalid secret");
        assert_eq!(secret, TEST_SECRET);
        assert_ne!(secret, "not-my-secret");
    }
}
