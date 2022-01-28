use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fmt::Formatter;

use pest::iterators::Pair;
use pest::Parser;

use crate::error::{Error, InputResolveError, Result};
use crate::{Config, Inputs, Value};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ConfigParser;

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Attempts to get an input value from the `inputs` map.
/// If the `key` starts with `$env_` the system environment variables will be consulted first.
fn get_input<'a>(key: &'a str, inputs: &Inputs<'a>) -> Result<Value<'a>> {
    if key.starts_with("$env_") {
        let env_name = key.replace("$env_", "");
        let pair = env::vars().find(|(key, _)| key == &env_name);
        if let Some(pair) = pair {
            return Ok(Value::String(pair.1));
        }
    }

    if let Some(value) = inputs.get(key) {
        // TODO: Not clone
        Ok(value.clone())
    } else {
        Err(Error::InputResolveError(InputResolveError(key.to_string())))

        // TODO: Only print errors & exit from binary
        // print_err(format!("Input `{}` was used but not declared", key), None);
        // exit(ERR_INPUT);
    }
}

/// Parses a pair of `Rule`s into a `Value`.
fn parse_value<'a>(pair: Pair<'a, Rule>, inputs: &Inputs<'a>) -> Result<Value<'a>> {
    match pair.as_rule() {
        Rule::object => Ok(Value::Object(parse_object(pair, inputs)?)),
        Rule::array => Ok(Value::Array(parse_array(pair, inputs)?)),
        Rule::string => Ok(Value::String(parse_string(pair))),
        Rule::integer => Ok(Value::Integer(pair.as_str().parse().unwrap())),
        Rule::float => Ok(Value::Float(pair.as_str().parse().unwrap())),
        Rule::boolean => Ok(Value::Boolean(pair.as_str().parse().unwrap())),
        Rule::null => Ok(Value::Null(None)),
        Rule::input => {
            let key = pair.as_str();
            get_input(key, inputs)
        }
        _ => unreachable!(),
    }
}

/// Adds `Value` at the `path` in `obj`.
///
/// `path` is an array where each entry represents another object key,
/// for example `foo.bar` is represented as `["foo", "bar"]`.
///
/// Objects are automatically created up to the required depth recursively.
fn add_at_path<'a>(
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
            obj.insert(part, Value::Object(add_at_path(map, path_rest, value)));
        }
        _ => panic!(
            "{} attempts to use object dot notation for a non-object type",
            path.join(".")
        ),
    };

    obj
}

/// Collects each `char` in a `Rule::string`
/// to form a single `String`.
fn parse_string(pair: Pair<Rule>) -> String {
    assert_eq!(pair.as_rule(), Rule::string);
    pair.into_inner()
        .map(|char| {
            let value = char.as_str();
            value.to_string()
        })
        .collect::<String>()
}

/// Parses each rule in a `Rule::array`
/// to form a vector of `Value`s.
fn parse_array<'a>(block: Pair<'a, Rule>, inputs: &Inputs<'a>) -> Result<Vec<Value<'a>>> {
    assert_eq!(block.as_rule(), Rule::array);
    block
        .into_inner()
        .map(|pair| parse_value(pair, inputs))
        .collect::<Result<Vec<_>>>()
}

/// Parses each key/value pair in a `Rule::object`
/// to form a BTreeMap of Values.
///
/// A BTreeMap is used to ensure keys
/// always output in the same order.
fn parse_object<'a>(
    block: Pair<'a, Rule>,
    inputs: &Inputs<'a>,
) -> Result<BTreeMap<&'a str, Value<'a>>> {
    assert_eq!(block.as_rule(), Rule::object);

    let mut obj = BTreeMap::new();

    for rule in block.into_inner() {
        match rule.as_rule() {
            Rule::pair => {
                let mut path_rules = rule.into_inner();
                let path = path_rules.next().unwrap().as_str();
                let value = parse_value(path_rules.next().unwrap(), inputs)?;

                obj = add_at_path(obj, path.split('.').collect::<Vec<_>>().as_slice(), value);
            }
            _ => unreachable!(),
        }
    }

    Ok(obj)
}

/// Parses the `let { } in` block at the start of files,
/// producing a populated HashMap as an output.
fn parse_assign_block(block: Pair<Rule>) -> Result<Inputs> {
    assert_eq!(block.as_rule(), Rule::assign_block);

    let mut inputs = HashMap::new();

    for pair in block.into_inner() {
        let mut assign_rules = pair.into_inner();
        let name = assign_rules.next().unwrap().as_str();
        let value = parse_value(assign_rules.next().unwrap(), &inputs)?;

        inputs.insert(name, value);
    }

    Ok(inputs)
}

/// Parses the input string into a `Config`
/// containing the resolved inputs
/// and a map of values representing the top-level object.
///
/// # Examples
///
/// ```rust
/// use cornfig::parse;
///
/// let corn = "{foo = 42}";
///
/// let config = parse(corn).unwrap();
/// let json = serde_json::to_string(&config.value).unwrap();
///
/// assert_eq!(json, "{\"foo\":42}");
/// ```
///
/// # Errors
///
/// TODO: Write about errors after improving error handling
///
///
pub fn parse(file: &str) -> Result<Config> {
    let rules = ConfigParser::parse(Rule::config, file);

    match rules {
        Ok(mut rules) => {
            let first_block = rules.next().unwrap();

            match first_block.as_rule() {
                Rule::assign_block => {
                    let value_block = rules.next().unwrap();

                    let inputs = parse_assign_block(first_block)?;
                    let value_block = parse_object(value_block, &inputs)?;

                    Ok(Config {
                        inputs,
                        value: Value::Object(value_block),
                    })
                }
                Rule::object => {
                    let inputs = HashMap::new();

                    let value_block = parse_object(first_block, &inputs)?;

                    Ok(Config {
                        inputs,
                        value: Value::Object(value_block),
                    })
                }
                _ => unreachable!(),
            }
        }
        Err(error) => Err(Error::ParserError(error)),
    }
}
