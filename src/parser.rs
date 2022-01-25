use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fmt::Formatter;
use std::process::exit;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

use crate::error::{print_err, ERR_VARIABLE};
use crate::{Config, Value, Variables};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct ConfigParser;

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn attempt_get_variable<'a>(key: &'a str, variables: &Variables<'a>) -> Value<'a> {
    if key.starts_with("$env_") {
        let env_name = key.replace("$env_", "");
        let pair = env::vars().find(|(key, _)| key == &env_name);
        if let Some(pair) = pair {
            return Value::String(pair.1);
        }
    }

    let value = variables.get(key);

    if let Some(value) = value {
        value.clone()
    } else {
        // TODO: Only print errors & exit from binary
        print_err(
            format!("Variable `{}` was used but not declared", key),
            None,
        );
        exit(ERR_VARIABLE);
    }
}

fn parse_value<'a>(pair: Pair<'a, Rule>, variables: &Variables<'a>) -> Value<'a> {
    match pair.as_rule() {
        Rule::object => Value::Object(parse_object(pair, variables)),
        Rule::array => Value::Array(parse_array(pair, variables)),
        Rule::string => Value::String(parse_string(pair, variables)),
        Rule::integer => Value::Integer(pair.as_str().parse().unwrap()),
        Rule::float => Value::Float(pair.as_str().parse().unwrap()),
        Rule::boolean => Value::Boolean(pair.as_str().parse().unwrap()),
        Rule::null => Value::Null,
        Rule::variable => {
            let key = pair.as_str();
            attempt_get_variable(key, variables)
        }
        _ => unreachable!(),
    }
}

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

fn parse_string<'a>(pair: Pair<'a, Rule>, variables: &Variables<'a>) -> String {
    pair.into_inner()
        .map(|char| {
            let value = char.as_str();
            return if value.len() == 1 {
                value.to_string()
            } else {
                let value = parse_value(char.into_inner().next().unwrap(), variables);
                match value {
                    Value::String(val) => val,
                    Value::Integer(val) => val.to_string(),
                    Value::Float(val) => val.to_string(),
                    Value::Boolean(val) => val.to_string(),
                    _ => unimplemented!(),
                }
            };
        })
        .collect::<String>()
}

fn parse_array<'a>(block: Pair<'a, Rule>, variables: &Variables<'a>) -> Vec<Value<'a>> {
    assert_eq!(block.as_rule(), Rule::array);
    block
        .into_inner()
        .map(|pair| parse_value(pair, variables))
        .collect::<Vec<_>>()
}

pub fn parse_object<'a>(
    block: Pair<'a, Rule>,
    variables: &Variables<'a>,
) -> BTreeMap<&'a str, Value<'a>> {
    assert_eq!(block.as_rule(), Rule::object);

    let mut obj = BTreeMap::new();

    for rule in block.into_inner() {
        match rule.as_rule() {
            Rule::pair => {
                let mut path_rules = rule.into_inner();
                let path = path_rules.next().unwrap().as_str();
                let value = parse_value(path_rules.next().unwrap(), variables);

                obj = add_at_path(obj, path.split('.').collect::<Vec<_>>().as_slice(), value);
            }
            _ => unreachable!(),
        }
    }

    obj
}

pub fn parse_assign_block(block: Pair<Rule>) -> Variables {
    assert_eq!(block.as_rule(), Rule::assign_block);

    let mut variables = HashMap::new();

    for pair in block.into_inner() {
        let mut assign_rules = pair.into_inner();
        let name = assign_rules.next().unwrap().as_str();
        let value = parse_value(assign_rules.next().unwrap(), &variables);

        variables.insert(name, value);
    }

    variables
}

pub fn parse(file: &str) -> Result<Config, Error<Rule>> {
    let rules = ConfigParser::parse(Rule::config, file);

    match rules {
        Ok(mut rules) => {
            let first_block = rules.next().unwrap();

            match first_block.as_rule() {
                Rule::assign_block => {
                    let value_block = rules.next().unwrap();

                    let variables = parse_assign_block(first_block);
                    let value_block = parse_object(value_block, &variables);

                    Ok(Config {
                        variables,
                        value: value_block,
                    })
                }
                Rule::object => {
                    let variables = HashMap::new();

                    let value_block = parse_object(first_block, &variables);

                    Ok(Config {
                        variables,
                        value: value_block,
                    })
                }
                _ => unreachable!(),
            }
        }
        Err(error) => Err(error),
    }
}
