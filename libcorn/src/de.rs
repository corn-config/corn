use std::collections::VecDeque;

use serde::de::{self, DeserializeSeed, EnumAccess, IntoDeserializer, VariantAccess, Visitor};

use crate::error::{Error, Result};
use crate::parse;
use crate::Value;

#[derive(Debug)]
pub struct Deserializer<'de> {
    value: Option<Value<'de>>,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Result<Self> {
        let parsed = parse(input)?;

        Ok(Self::from_value(parsed))
    }

    fn from_value(value: Value<'de>) -> Self {
        Self { value: Some(value) }
    }
}

/// Attempts to deserialize the config from a string slice.
///
/// # Errors
///
/// Will return a `DeserializationError` if the config is invalid.
pub fn from_str<T>(s: &str) -> Result<T>
where
    T: de::DeserializeOwned,
{
    let mut deserializer = Deserializer::from_str(s)?;
    T::deserialize(&mut deserializer)
}

/// Attempts to deserialize the config from a byte slice.
///
/// # Errors
///
/// Will return a `DeserializationError` if the config is invalid.
pub fn from_slice<T>(bytes: &[u8]) -> Result<T>
where
    T: de::DeserializeOwned,
{
    match std::str::from_utf8(bytes) {
        Ok(s) => from_str(s),
        Err(e) => Err(Error::DeserializationError(e.to_string())),
    }
}

macro_rules! get_value {
    ($self:ident) => {
        match $self.value.take() {
            Some(val) => Ok(val),
            None => Err(Error::DeserializationError(String::from(
                "Deserializer value unexpectedly `None`",
            ))),
        }?
    };
}

macro_rules! err_expected {
    ($expected:literal, $got:expr) => {
        Err(Error::DeserializationError(format!(
            "Expected {}, found '{:?}'",
            $expected, $got
        )))
    };
}

macro_rules! match_value {
    ($self:ident, $name:literal, $($pat:pat => $expr:expr)+) => {{
        let value = get_value!($self);
        match value {
            $($pat => $expr, )+
            _ => err_expected!($name, value)
        }
    }};
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = get_value!(self);
        match value {
            Value::Object(_) => {
                let map = Map::new(value);
                visitor.visit_map(map)
            }
            Value::Array(_) => {
                let seq = Seq::new(value);
                visitor.visit_seq(seq)
            }
            Value::String(val) => visitor.visit_str(&val),
            Value::Integer(val) => visitor.visit_i64(val),
            Value::Float(val) => visitor.visit_f64(val),
            Value::Boolean(val) => visitor.visit_bool(val),
            Value::Null(_) => visitor.visit_unit(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "boolean", Value::Boolean(val) => visitor.visit_bool(val))
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (i8)", Value::Integer(val) =>  visitor.visit_i8(val as i8))
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (i16)", Value::Integer(val) =>  visitor.visit_i16(val as i16))
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (i32)", Value::Integer(val) =>  visitor.visit_i32(val as i32))
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (i64)", Value::Integer(val) =>  visitor.visit_i64(val))
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (u8)", Value::Integer(val) =>  visitor.visit_u8(val as u8))
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (u16)", Value::Integer(val) =>  visitor.visit_u16(val as u16))
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (u32)", Value::Integer(val) =>  visitor.visit_u32(val as u32))
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "integer (u64)", Value::Integer(val) =>  visitor.visit_u64(val as u64))
    }

    fn deserialize_f32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "float (f32)", Value::Float(val) =>  visitor.visit_f32(val as f32))
    }

    fn deserialize_f64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "float (f64)", Value::Float(val) =>  visitor.visit_f64(val))
    }

    fn deserialize_char<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = get_value!(self);
        let char = match value {
            Value::String(value) => value.chars().next(),
            _ => return err_expected!("char", value),
        };

        match char {
            Some(char) => visitor.visit_char(char),
            None => err_expected!("char", "empty string"),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "string",
            Value::String(val) => visitor.visit_str(&val)
        )
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match_value!(self, "bytes array",
            Value::String(val) => visitor.visit_bytes(val.as_bytes())
        )
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = get_value!(self);
        match value {
            Value::Null(_) => visitor.visit_none(),
            _ => visitor.visit_some(&mut Deserializer::from_value(value)),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = get_value!(self);
        match value {
            Value::Array(_) => visitor.visit_seq(Seq::new(value)),
            _ => err_expected!("array", value),
        }
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = get_value!(self);
        match value {
            Value::Object(_) => visitor.visit_map(Map::new(value)),
            _ => err_expected!("object", value),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = get_value!(self);
        match value {
            Value::Object(_) => visitor.visit_enum(Enum::new(value)),
            Value::String(val) => visitor.visit_enum(val.into_deserializer()),
            _ => err_expected!("object or string (enum variant)", value),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct Map<'de> {
    values: VecDeque<Value<'de>>,
}

impl<'de> Map<'de> {
    fn new(value: Value<'de>) -> Self {
        match value {
            Value::Object(values) => Self {
                values: values
                    .into_iter()
                    .flat_map(|(key, value)| vec![Value::String(key), value])
                    .collect(),
            },
            _ => unreachable!(),
        }
    }
}

impl<'de> de::MapAccess<'de> for Map<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(value) = self.values.pop_front() {
            seed.deserialize(&mut Deserializer::from_value(value))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.values.pop_front() {
            Some(value) => seed.deserialize(&mut Deserializer::from_value(value)),
            None => Err(Error::DeserializationError(
                "Expected value to exist".to_string(),
            )),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.values.len() / 2)
    }
}

struct Seq<'de> {
    values: VecDeque<Value<'de>>,
}

impl<'de> Seq<'de> {
    fn new(value: Value<'de>) -> Self {
        match value {
            Value::Array(values) => Self {
                values: VecDeque::from(values),
            },
            _ => unreachable!(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for Seq<'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.values.pop_front() {
            seed.deserialize(&mut Deserializer::from_value(value))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.values.len())
    }
}

struct Enum<'de> {
    value: Value<'de>,
}

impl<'de> Enum<'de> {
    fn new(value: Value<'de>) -> Self {
        Self { value }
    }
}

impl<'de> EnumAccess<'de> for Enum<'de> {
    type Error = Error;
    type Variant = Variant<'de>;

    fn variant_seed<V>(self, seed: V) -> std::result::Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value {
            Value::String(_) => {
                let value = seed.deserialize(&mut Deserializer::from_value(self.value))?;
                Ok((value, Variant::new(None)))
            }
            Value::Object(obj) => {
                let first_pair = obj.into_iter().next();
                if let Some(first_pair) = first_pair {
                    let value = Value::String(first_pair.0);
                    let tag = seed.deserialize(&mut Deserializer::from_value(value))?;
                    Ok((tag, Variant::new(Some(first_pair.1))))
                } else {
                    Err(Error::DeserializationError(
                        "Cannot deserialize empty object into enum".to_string(),
                    ))
                }
            }
            _ => unreachable!(),
        }
    }
}

struct Variant<'de> {
    value: Option<Value<'de>>,
}

impl<'de> Variant<'de> {
    fn new(value: Option<Value<'de>>) -> Self {
        Self { value }
    }
}

impl<'de> VariantAccess<'de> for Variant<'de> {
    type Error = Error;

    fn unit_variant(self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> std::result::Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(&mut Deserializer::from_value(value)),
            None => Err(Error::DeserializationError(
                "Expected value to exist".to_string(),
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(value) if matches!(value, Value::Array(_)) => visitor.visit_seq(Seq::new(value)),
            _ => unreachable!(),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(value) if matches!(value, Value::Object(_)) => visitor.visit_map(Map::new(value)),
            _ => unreachable!(),
        }
    }
}
