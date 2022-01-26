use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

pub use crate::parser::parse;
pub(crate) use crate::parser::Rule;

pub mod error;
mod parser;

/// A map of input names and values.
/// The names include their `$` prefix.
pub type Inputs<'a> = HashMap<&'a str, Value<'a>>;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value<'a> {
    Object(BTreeMap<&'a str, Value<'a>>),
    Array(Vec<Value<'a>>),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Serialize, Debug)]
pub struct Config<'a> {
    pub inputs: Inputs<'a>,
    pub value: BTreeMap<&'a str, Value<'a>>,
}
