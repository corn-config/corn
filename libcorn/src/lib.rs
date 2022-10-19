use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

pub use crate::parser::{parse, Rule};

pub mod error;
mod parser;

#[cfg(feature = "wasm")]
mod wasm;

/// A map of input names and values.
/// The names include their `$` prefix.
pub type Inputs<'a> = HashMap<&'a str, Value<'a>>;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value<'a> {
    // #[serde(serialize_with = "toml::ser::tables_last")]
    // #[serde(serialize_with = "serialize")]
    Object(BTreeMap<&'a str, Value<'a>>),
    Array(Vec<Value<'a>>),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null(Option<()>),
}

#[derive(Serialize, Debug)]
pub struct Config<'a> {
    pub inputs: Inputs<'a>,
    pub value: Value<'a>,
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
            Value::String(val) => TomlValue::String(val),
            Value::Integer(val) => TomlValue::Integer(val),
            Value::Float(val) => TomlValue::Float(val),
            Value::Boolean(val) => TomlValue::Boolean(val),
            Value::Null(_) => unreachable!(),
        }
    }
}
