use std::collections::{BTreeMap, HashMap};
use std::env::var;
use std::fmt::Formatter;

use pest::iterators::Pair;
use pest::Parser;

use crate::error::{Error, InputResolveError, Result};
use crate::{Inputs, Value};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct AstParser;

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
            self.parse_assign_block(input_block).unwrap()
        }

        let value_block = self.parse_object(object_block)?;
        Ok(Value::Object(value_block))
    }

    /// Parses a pair of tokens (marked as a `Rule`) into a `Value`.
    fn parse_value(&self, pair: Pair<'a, Rule>) -> Result<Value<'a>> {
        match pair.as_rule() {
            Rule::object => Ok(Value::Object(self.parse_object(pair)?)),
            Rule::array => Ok(Value::Array(self.parse_array(pair)?)),
            Rule::string => Ok(Value::String(self.parse_string(pair))),
            Rule::integer => Ok(Value::Integer(self.parse_integer(&pair))),
            Rule::float => Ok(Value::Float(self.parse_float(&pair))),
            Rule::boolean => Ok(Value::Boolean(self.parse_bool(&pair))),
            Rule::null => Ok(Value::Null(None)),
            Rule::input => {
                let key = pair.as_str();
                self.get_input(key)
            }
            _ => unreachable!(),
        }
    }

    fn parse_bool(&self, pair: &Pair<'_, Rule>) -> bool {
        assert_eq!(pair.as_rule(), Rule::boolean);
        match pair.as_str() {
            "true" => true,
            "false" => false,
            _ => unreachable!(),
        }
    }

    fn parse_integer(&self, pair: &Pair<'_, Rule>) -> i64 {
        assert_eq!(pair.as_rule(), Rule::integer);
        pair.as_str().parse().unwrap()
    }

    fn parse_float(&self, pair: &Pair<'_, Rule>) -> f64 {
        assert_eq!(pair.as_rule(), Rule::float);
        pair.as_str().parse().unwrap()
    }

    /// Collects each `char` in a `Rule::string`
    /// to form a single `String`.
    fn parse_string(&self, pair: Pair<'a, Rule>) -> &'a str {
        assert_eq!(pair.as_rule(), Rule::string);
        pair.into_inner().next().unwrap().as_str()
    }

    /// Parses each rule in a `Rule::array`
    /// to form a vector of `Value`s.
    fn parse_array(&self, block: Pair<'a, Rule>) -> Result<Vec<Value<'a>>> {
        assert_eq!(block.as_rule(), Rule::array);
        block
            .into_inner()
            .map(|pair| self.parse_value(pair))
            .collect::<Result<Vec<_>>>()
    }

    /// Parses each key/value pair in a `Rule::object`
    /// to form a BTreeMap of Values.
    ///
    /// A BTreeMap is used to ensure keys
    /// always output in the same order.
    fn parse_object(&self, block: Pair<'a, Rule>) -> Result<BTreeMap<&'a str, Value<'a>>> {
        assert_eq!(block.as_rule(), Rule::object);

        let mut obj = BTreeMap::new();

        for pair in block.into_inner() {
            match pair.as_rule() {
                Rule::pair => {
                    let mut path_rules = pair.into_inner();
                    let path = path_rules.next().unwrap().as_str();
                    let value = self.parse_value(path_rules.next().unwrap())?;

                    obj = Self::add_at_path(
                        obj,
                        path.split('.').collect::<Vec<_>>().as_slice(),
                        value,
                    );
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
    ) -> BTreeMap<&'a str, Value<'a>> {
        let (part, path_rest) = path.split_first().unwrap();

        if path_rest.is_empty() {
            obj.insert(part, value);
            return obj;
        }

        if !obj.contains_key(part) {
            obj.insert(part, Value::Object(BTreeMap::new()));
        }

        let child_obj = obj.get(part).unwrap().clone(); // TODO: Not clone

        match child_obj {
            Value::Object(map) => {
                obj.insert(
                    part,
                    Value::Object(Self::add_at_path(map, path_rest, value)),
                );
            }
            _ => panic!(
                "{} attempts to use object dot notation for a non-object type",
                path.join(".")
            ),
        };

        obj
    }

    /// Parses the `let { } in` block at the start of files,
    /// producing a populated HashMap as an output.
    fn parse_assign_block(&mut self, block: Pair<'a, Rule>) -> Result<()> {
        assert_eq!(block.as_rule(), Rule::assign_block);

        for pair in block.into_inner() {
            let mut assign_rules = pair.into_inner();
            let name = assign_rules.next().unwrap().as_str();

            let value = self.parse_value(assign_rules.next().unwrap())?;

            self.inputs.insert(name, value);
        }

        Ok(())
    }

    /// Attempts to get an input value from the `inputs` map.
    /// If the `key` starts with `$env_` the system environment variables will be consulted first.
    fn get_input(&self, key: &'a str) -> Result<Value<'a>> {
        if key.starts_with("$env_") {
            let env_name = key.replace("$env_", "");
            let var = var(env_name);

            if let Ok(var) = var {
                return Ok(Value::EnvString(var));
            }
        }

        if let Some(value) = self.inputs.get(key) {
            Ok(value.clone())
        } else {
            Err(Error::InputResolveError(InputResolveError(key.to_string())))

            // TODO: Only print errors & exit from binary
            // print_err(format!("Input `{}` was used but not declared", key), None);
            // exit(ERR_INPUT);
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
/// TODO: Write about errors after improving error handling
///
///
pub fn parse(file: &str) -> Result<Value> {
    let rules = AstParser::parse(Rule::config, file);

    match rules {
        Ok(mut rules) => {
            let first_block = rules.next().unwrap();

            match first_block.as_rule() {
                Rule::assign_block => {
                    let parser = CornParser::new(Some(first_block));
                    let object_block = rules.next().unwrap();
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
