use poem_openapi::registry::MetaSchemaRef;
use poem_openapi::types::{ParseError, ParseFromJSON, ToJSON, Type};
use serde_json::{Map, Value};
use std::borrow::Cow;

use crate::Secret;

trait AsJson {
    fn as_json(&self) -> String;
}

#[cfg(not(feature = "base64"))]
impl AsJson for Secret {
    fn as_json(&self) -> String {
        self.to_string()
    }
}

#[cfg(feature = "base64")]
impl AsJson for Secret {
    fn as_json(&self) -> String {
        self.to_base64()
    }
}

impl Type for Secret {
    const IS_REQUIRED: bool = true;
    type RawValueType = Self;
    type RawElementValueType = Self;

    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Secret")
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Reference(Self::name().to_string())
    }

    fn register(registry: &mut poem_openapi::registry::Registry) {
        registry.create_schema::<Self, _>(Self::name().to_string(), |registry| {
            <String as Type>::register(registry);
            let mut meta = poem_openapi::registry::MetaSchema {
                description: Some("Wrapper for storing secrets in a secure manner"),
                external_docs: None,
                required: vec!["secret"],
                properties: {
                    let mut fields = Vec::new();
                    {
                        let original_schema = <String as Type>::schema_ref();
                        let patch_schema = {
                            let mut schema = poem_openapi::registry::MetaSchema::ANY;
                            schema.default = None;
                            schema.read_only = false;
                            schema.write_only = false;

                            schema
                        };
                        fields.push(("secret", original_schema.merge(patch_schema)));
                    }
                    fields
                },
                deprecated: false,
                ..poem_openapi::registry::MetaSchema::new("object")
            };
            meta.example = None;
            meta
        })
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(std::iter::IntoIterator::into_iter(self.as_raw_value()))
    }
}

impl ParseFromJSON for Secret {
    fn parse_from_json(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        let value = value.unwrap_or_default();

        match value {
            Value::Object(mut obj) => {
                let secret: String = ParseFromJSON::parse_from_json(obj.remove("secret"))
                    .map_err(ParseError::propagate)?;
                Self::wrap(secret).map_err(ParseError::custom)
            }
            _ => Err(ParseError::expected_type(value)),
        }
    }
}

impl ToJSON for Secret {
    fn to_json(&self) -> Option<Value> {
        let mut object = Map::new();
        if let Some(value) = ToJSON::to_json(&self.as_json()) {
            object.insert(String::from("secret"), value);
        }
        Some(Value::Object(object))
    }
}
