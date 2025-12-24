use alloc::{string::String, vec::Vec};
use serde_core::{Deserialize, de::Visitor};

use crate::{Object, Value};

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("any valid Corn value")
            }

            #[inline]
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                Ok(Value::Boolean(v))
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                Ok(Value::Integer(v.into()))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                Ok(Value::Integer(v.into()))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                Ok(Value::Float(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                self.visit_string(String::from(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                Ok(Value::String(v))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                Ok(Value::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde_core::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde_core::de::Error,
            {
                Ok(Value::Null)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde_core::de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::Array(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde_core::de::MapAccess<'de>,
            {
                let mut dict = Object::default();

                while let Some((key, value)) = map.next_entry()? {
                    dict.insert(key, value);
                }

                Ok(Value::Object(dict))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
