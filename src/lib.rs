use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

pub use crate::parser::{parse, Rule};

pub mod error;
mod parser;

pub type Variables<'a> = HashMap<&'a str, Value<'a>>;

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
    pub variables: Variables<'a>,
    pub value: BTreeMap<&'a str, Value<'a>>,
}
