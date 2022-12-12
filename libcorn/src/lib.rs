use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};

pub use crate::de::{from_slice, from_str};
pub use crate::parser::{parse, Rule};

pub mod error;
mod parser;

mod de;
#[cfg(feature = "wasm")]
mod wasm;

/// A map of input names and values.
/// The names include their `$` prefix.
pub type Inputs<'a> = HashMap<&'a str, Value<'a>>;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value<'a> {
    /// Key/value map. Values can be mixed types.
    Object(BTreeMap<&'a str, Value<'a>>),
    /// Array of values, can be mixed types.
    Array(Vec<Value<'a>>),
    /// Borrowed string, from string literal or input.
    String(&'a str),
    /// Owned string, originating from an environment variable.
    EnvString(String),
    /// 64-bit signed integer.
    Integer(i64),
    /// 64-bit (double precision) floating point number.
    Float(f64),
    /// true or false
    Boolean(bool),
    /// `null` literal.
    Null(Option<()>),
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Value::Object(_) => "object",
            Value::Array(_) => "array",
            Value::String(_) => "string",
            Value::EnvString(_) => "string (from environment variable)",
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::Boolean(_) => "boolean",
            Value::Null(_) => "null"
        })
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum TomlValue {
    #[serde(serialize_with = "toml::ser::tables_last")]
    Object(BTreeMap<String, TomlValue>),
    Array(Vec<TomlValue>),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl From<Value<'_>> for TomlValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(val) => {
                let obj = val
                    .iter()
                    .filter_map(|(k, v)| {
                        if let Value::Null(_) = v {
                            None
                        } else {
                            Some((k.to_string(), TomlValue::from(v.clone())))
                        }
                    })
                    .collect();

                TomlValue::Object(obj)
            }
            Value::Array(val) => {
                let arr = val
                    .iter()
                    .filter_map(|v| {
                        if let Value::Null(_) = v {
                            None
                        } else {
                            Some(TomlValue::from(v.clone()))
                        }
                    })
                    .collect();
                TomlValue::Array(arr)
            }
            Value::String(val) => TomlValue::String(val.to_string()),
            Value::EnvString(val) => TomlValue::String(val),
            Value::Integer(val) => TomlValue::Integer(val),
            Value::Float(val) => TomlValue::Float(val),
            Value::Boolean(val) => TomlValue::Boolean(val),
            Value::Null(_) => unreachable!(),
        }
    }
}
