use serde_core::Serialize;

use crate::{BorrowedValue, Value};

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            Self::String(s) => serializer.serialize_str(s),
            Self::Integer(i) => i.serialize(serializer),
            Self::Float(f) => f.serialize(serializer),
            Self::Boolean(v) => serializer.serialize_bool(*v),
            Self::Object(obj) => {
                use serde_core::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(obj.len()))?;

                for (k, v) in obj {
                    map.serialize_entry(k, v)?;
                }

                map.end()
            }
            Self::Array(v) => v.serialize(serializer),
            Self::Null => serializer.serialize_none(),
        }
    }
}

impl Serialize for BorrowedValue<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self {
            Self::String(s) => serializer.serialize_str(s),
            Self::Integer(i) => i.serialize(serializer),
            Self::Float(f) => f.serialize(serializer),
            Self::Boolean(v) => serializer.serialize_bool(*v),
            Self::Object(obj) => {
                use serde_core::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(obj.len()))?;

                for (k, v) in obj {
                    map.serialize_entry(k, v)?;
                }

                map.end()
            }
            Self::Array(v) => v.serialize(serializer),
            Self::Null => serializer.serialize_none(),
        }
    }
}
