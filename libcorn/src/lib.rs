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
        write!(
            f,
            "{}",
            match self {
                Value::Object(_) => "object",
                Value::Array(_) => "array",
                Value::String(_) => "string",
                Value::EnvString(_) => "string (from environment variable)",
                Value::Integer(_) => "integer",
                Value::Float(_) => "float",
                Value::Boolean(_) => "boolean",
                Value::Null(_) => "null",
            }
        )
    }
}
