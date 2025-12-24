use core::fmt::{Debug, Display};

use serde_core::{
    Deserialize, Serialize,
    de::{self, Visitor},
    forward_to_deserialize_any,
};

use crate::Error;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct Integer {
    inner: IntegerType,
}

impl Debug for Integer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Integer({self})")
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.inner {
            IntegerType::Signed(n) => f.write_str(itoa::Buffer::new().format(n)),
            IntegerType::Unsigned(n) => f.write_str(itoa::Buffer::new().format(n)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum IntegerType {
    Signed(i64),
    Unsigned(u64),
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self {
            inner: IntegerType::Signed(value),
        }
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Self {
            inner: IntegerType::Unsigned(value),
        }
    }
}

impl Integer {
    pub const fn is_i64(&self) -> bool {
        match self.inner {
            IntegerType::Unsigned(n) => n <= i64::MAX as u64,
            IntegerType::Signed(_) => true,
        }
    }

    pub const fn is_u64(&self) -> bool {
        matches!(self.inner, IntegerType::Unsigned(_))
    }

    pub const fn as_i64(&self) -> Option<i64> {
        match self.inner {
            IntegerType::Signed(n) => Some(n),
            IntegerType::Unsigned(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
        }
    }

    pub const fn as_u64(&self) -> Option<u64> {
        match self.inner {
            IntegerType::Unsigned(n) => Some(n),
            IntegerType::Signed(_) => None,
        }
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        match self.inner {
            IntegerType::Signed(integer) => serializer.serialize_i64(integer),
            IntegerType::Unsigned(integer) => serializer.serialize_u64(integer),
        }
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct IntegerVisitor;

        impl<'de> Visitor<'de> for IntegerVisitor {
            type Value = Integer;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("an Integer")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(value.into())
            }
        }

        deserializer.deserialize_any(IntegerVisitor)
    }
}

impl<'de> de::Deserializer<'de> for Integer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.inner {
            IntegerType::Signed(integer) => visitor.visit_i64(integer),
            IntegerType::Unsigned(integer) => visitor.visit_u64(integer),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
