extern crate core;

use cornfig::{parse, TomlValue};
use paste::paste;
use std::fs;

macro_rules! generate_eq_tests {
    ($($test_name:ident),+) => {
        $(
            paste!{
                #[test]
                fn [<json_ $test_name>]() {
                    let test_name = stringify!($test_name);

                    let input = fs::read_to_string(format!("assets/inputs/{}.corn", test_name)).unwrap();
                    let valid = fs::read_to_string(format!("assets/outputs/json/{}.json", test_name)).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let serialized = serde_json::to_string_pretty(&config.value).unwrap().replace("\r", "");

                    assert_eq!(serialized.trim(), valid.trim());
                }

                #[test]
                fn [<yaml_ $test_name>]() {
                    let test_name = stringify!($test_name);

                    let input = fs::read_to_string(format!("assets/inputs/{}.corn", test_name)).unwrap();
                    let valid = fs::read_to_string(format!("assets/outputs/yaml/{}.yml", test_name)).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let serialized = serde_yaml::to_string(&config.value).unwrap().replace("\r", "");

                    assert_eq!(serialized.trim(), valid.trim());
                }

                #[test]
                fn [<toml_ $test_name>]() {
                    let test_name = stringify!($test_name);

                    let input = fs::read_to_string(format!("assets/inputs/{}.corn", test_name)).unwrap();
                    let valid = fs::read_to_string(format!("assets/outputs/toml/{}.toml", test_name)).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let value = TomlValue::from(config.value);

                    let serialized = toml::to_string_pretty(&value).unwrap().replace("\r", "");

                    assert_eq!(serialized.trim(), valid.trim());
                }
            }

        )+
    }
}

macro_rules! generate_invalid_tests {
    ($($test_name:ident),+) => {
        $(
            #[test]
            fn $test_name() {
                let test_name = stringify!($test_name);

                let input = fs::read_to_string(format!("assets/inputs/{}.corn", test_name)).unwrap();

                let config = parse(input.as_str());
                assert!(config.is_err());
            }
        )+
    }
}

generate_eq_tests!(
    basic,
    basic_empty_let,
    string,
    integer,
    float,
    boolean,
    null,
    object,
    array,
    input,
    environment_variable,
    chained,
    chained_complex,
    compact,
    very_compact,
    comment,
    complex,
    readme_example,
    object_in_array,
    value_after_table
);

generate_invalid_tests!(invalid, invalid_input);
