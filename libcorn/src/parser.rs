use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::env::var;
use std::fmt::Formatter;

use pest::iterators::Pair;
use pest::Parser;

use crate::error::{Error, Result};
use crate::{Inputs, Value};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct AstParser;

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

struct CornParser<'a> {
    input_block: Option<Pair<'a, Rule>>,
    inputs: Inputs<'a>,
}

impl<'a> CornParser<'a> {
    pub fn new(input_block: Option<Pair<'a, Rule>>) -> Self {
        let inputs = HashMap::new();
        Self {
            input_block,
            inputs,
        }
    }

    pub fn parse(mut self, object_block: Pair<'a, Rule>) -> Result<Value> {
        if let Some(input_block) = self.input_block.take() {
            self.parse_assign_block(input_block)?;
        }

        let value_block = self.parse_object(object_block)?;
        Ok(Value::Object(value_block))
    }

    /// Parses a pair of tokens (marked as a `Rule`) into a `Value`.
    fn parse_value(&self, pair: Pair<'a, Rule>) -> Result<Value<'a>> {
        match pair.as_rule() {
            Rule::object => Ok(Value::Object(self.parse_object(pair)?)),
            Rule::array => Ok(Value::Array(self.parse_array(pair)?)),
            Rule::string => Ok(Value::String(self.parse_string(pair)?)),
            Rule::integer => Ok(Value::Integer(Self::parse_integer(pair))),
            Rule::float => Ok(Value::Float(Self::parse_float(&pair))),
            Rule::boolean => Ok(Value::Boolean(Self::parse_bool(&pair))),
            Rule::null => Ok(Value::Null(None)),
            Rule::input => {
                let key = pair.as_str();
                self.get_input(key)
            }
            _ => unreachable!(),
        }
    }

    fn parse_bool(pair: &Pair<'_, Rule>) -> bool {
        assert_eq!(pair.as_rule(), Rule::boolean);
        match pair.as_str() {
            "true" => true,
            "false" => false,
            _ => unreachable!(),
        }
    }

    fn parse_integer(pair: Pair<'_, Rule>) -> i64 {
        assert_eq!(pair.as_rule(), Rule::integer);
        let sub_pair = pair
            .into_inner()
            .next()
            .expect("integers should contain a sub-rule of their type");

        match sub_pair.as_rule() {
            Rule::decimal_integer => sub_pair
                .as_str()
                .replace('_', "")
                .parse()
                .expect("decimal integer rules should match valid rust integers"),
            Rule::hex_integer => i64::from_str_radix(&sub_pair.as_str()[2..], 16)
                .expect("hex integer rules contain valid hex values"),
            _ => unreachable!(),
        }
    }

    fn parse_float(pair: &Pair<'_, Rule>) -> f64 {
        assert_eq!(pair.as_rule(), Rule::float);
        pair.as_str()
            .parse()
            .expect("float rules should match valid rust floats")
    }

    /// Collects each `char` in a `Rule::string`
    /// to form a single `String`.
    fn parse_string(&self, pair: Pair<'a, Rule>) -> Result<Cow<'a, str>> {
        assert_eq!(pair.as_rule(), Rule::string);

        let mut full_string = String::new();

        let pairs = pair
            .into_inner()
            .next()
            .expect("string rules should contain a valid string value")
            .into_inner();

        for pair in pairs {
            match pair.as_rule() {
                Rule::char => full_string.push(Self::parse_char(&pair)),
                Rule::input => {
                    let input_name = pair.as_str();
                    let value = self.get_input(input_name)?;
                    match value {
                        Value::String(value) => full_string.push_str(&value),
                        _ => return Err(Error::InvalidInterpolationError(input_name.to_string())),
                    }
                }
                _ => unreachable!(),
            };
        }

        Ok(Cow::Owned(full_string))
    }

    fn parse_char(pair: &Pair<'a, Rule>) -> char {
        let str = pair.as_str();
        let mut chars = str.chars();

        let first_char = chars.next().expect("character to exist");
        if first_char != '\\' {
            return first_char;
        }

        let second_char = chars.next().expect("character to exist");
        if second_char != 'u' {
            return match second_char {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '"' => '\"',
                '\\' => '\\',
                _ => unreachable!(),
            };
        }

        let num =
            u32::from_str_radix(&str[3..], 16).expect("valid hex characters to exist after \\u");
        char::from_u32(num).unwrap_or('\u{FFFD}')
    }

    /// Parses each rule in a `Rule::array`
    /// to form a vector of `Value`s.
    fn parse_array(&self, block: Pair<'a, Rule>) -> Result<Vec<Value<'a>>> {
        assert_eq!(block.as_rule(), Rule::array);

        let mut arr = vec![];

        for pair in block.into_inner() {
            match pair.as_rule() {
                Rule::spread => {
                    let input = pair
                        .into_inner()
                        .next()
                        .expect("spread operators should contain an input");

                    let input_name = input.as_str();
                    let value = self.parse_value(input)?;

                    match value {
                        Value::Array(other) => arr.extend(other),
                        _ => return Err(Error::InvalidSpreadError(input_name.to_string())),
                    }
                }
                _ => arr.push(self.parse_value(pair)?),
            };
        }

        Ok(arr)
    }

    /// Parses each key/value pair in a `Rule::object`
    /// to form a `BTreeMap` of Values.
    ///
    /// A `BTreeMap` is used to ensure keys
    /// always output in the same order.
    fn parse_object(&self, block: Pair<'a, Rule>) -> Result<BTreeMap<&'a str, Value<'a>>> {
        assert_eq!(block.as_rule(), Rule::object);

        let mut obj = BTreeMap::new();

        for pair in block.into_inner() {
            match pair.as_rule() {
                Rule::pair => {
                    let mut path_rules = pair.into_inner();
                    let path = path_rules
                        .next()
                        .expect("object pairs should contain a key")
                        .as_str();
                    let value = self.parse_value(
                        path_rules
                            .next()
                            .expect("object pairs should contain a value"),
                    )?;

                    obj = Self::add_at_path(
                        obj,
                        path.split('.').collect::<Vec<_>>().as_slice(),
                        value,
                    )?;
                }
                Rule::spread => {
                    let input = pair
                        .into_inner()
                        .next()
                        .expect("spread operators should contain an input");

                    let input_name = input.as_str();
                    let value = self.parse_value(input)?;

                    match value {
                        Value::Object(other) => obj.extend(other),
                        _ => return Err(Error::InvalidSpreadError(input_name.to_string())),
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(obj)
    }

    /// Adds `Value` at the `path` in `obj`.
    ///
    /// `path` is an array where each entry represents another object key,
    /// for example `foo.bar` is represented as `["foo", "bar"]`.
    ///
    /// Objects are automatically created up to the required depth recursively.
    fn add_at_path(
        mut obj: BTreeMap<&'a str, Value<'a>>,
        path: &[&'a str],
        value: Value<'a>,
    ) -> Result<BTreeMap<&'a str, Value<'a>>> {
        let (part, path_rest) = path
            .split_first()
            .expect("paths should contain at least 1 segment");

        if path_rest.is_empty() {
            obj.insert(part, value);
            return Ok(obj);
        }

        let child_obj = obj
            .remove(part)
            .unwrap_or_else(|| Value::Object(BTreeMap::new()));

        match child_obj {
            Value::Object(map) => {
                obj.insert(
                    part,
                    Value::Object(Self::add_at_path(map, path_rest, value)?),
                );

                Ok(obj)
            }
            _ => Err(Error::InvalidPathError(path.join("."))),
        }
    }

    /// Parses the `let { } in` block at the start of files.
    /// Each input is inserted into into `self.inputs`.
    fn parse_assign_block(&mut self, block: Pair<'a, Rule>) -> Result<()> {
        assert_eq!(block.as_rule(), Rule::assign_block);

        for pair in block.into_inner() {
            let mut assign_rules = pair.into_inner();
            let name = assign_rules
                .next()
                .expect("input assignments should have a name")
                .as_str();

            let value = self.parse_value(
                assign_rules
                    .next()
                    .expect("input assignments should have a value"),
            )?;

            self.inputs.insert(name, value);
        }

        Ok(())
    }

    /// Attempts to get an input value from the `inputs` map.
    /// If the `key` starts with `$env_` the system environment variables will be consulted first.
    fn get_input(&self, key: &'a str) -> Result<Value<'a>> {
        if let Some(env_name) = key.strip_prefix("$env_") {
            let var = var(env_name);

            if let Ok(var) = var {
                return Ok(Value::String(Cow::Owned(var)));
            }
        }

        if let Some(value) = self.inputs.get(key) {
            Ok(value.clone())
        } else {
            Err(Error::InputResolveError(key.to_string()))
        }
    }
}

/// Parses the input string into a `Config`
/// containing the resolved inputs
/// and a map of values representing the top-level object.
///
/// # Examples
///
/// ```rust
/// use libcorn::parse;
///
/// let corn = "{foo = 42}";
///
/// let config = parse(corn).unwrap();
/// let json = serde_json::to_string(&config).unwrap();
///
/// assert_eq!(json, "{\"foo\":42}");
/// ```
///
/// # Errors
///
/// Will fail if the input contains a syntax error.
/// Will fail if the input contains invalid Corn for another reason,
/// including references to undefined inputs or dot-notation for non-object values.
/// Will fail if the input cannot be deserialized for any reaon.
///
/// Any of the above will return a specific error type with details.
pub fn parse(file: &str) -> Result<Value> {
    let rules = AstParser::parse(Rule::config, file);

    match rules {
        Ok(mut rules) => {
            let first_block = rules.next().expect("should be at least 1 rule");

            match first_block.as_rule() {
                Rule::assign_block => {
                    let parser = CornParser::new(Some(first_block));
                    let object_block = rules.next().expect("should always be an object block");
                    parser.parse(object_block)
                }
                Rule::object => {
                    let parser = CornParser::new(None);
                    parser.parse(first_block)
                }
                _ => unreachable!(),
            }
        }
        Err(error) => Err(Error::ParserError(Box::new(error))),
    }
}
