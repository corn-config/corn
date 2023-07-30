extern crate core;

use corn::parse;
use paste::paste;
use std::fs;

macro_rules! generate_eq_tests {
    ($($test_name:ident),+) => {
        $(
            paste!{
                #[test]
                fn [<json_ $test_name>]() {
                    let test_name = stringify!($test_name);

                    let input = fs::read_to_string(format!("../assets/inputs/{}.corn", test_name)).unwrap();
                    let valid = fs::read_to_string(format!("../assets/outputs/json/{}.json", test_name)).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let serialized = serde_json::to_string_pretty(&config).unwrap().replace("\r", "");

                    assert_eq!(serialized.trim(), valid.trim());
                }

                #[test]
                fn [<yaml_ $test_name>]() {
                    let test_name = stringify!($test_name);

                    let input = fs::read_to_string(format!("../assets/inputs/{}.corn", test_name)).unwrap();
                    let valid = fs::read_to_string(format!("../assets/outputs/yaml/{}.yml", test_name)).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let serialized = serde_yaml::to_string(&config).unwrap().replace("\r", "");

                    assert_eq!(serialized.trim(), valid.trim());
                }

                #[test]
                fn [<toml_ $test_name>]() {
                    let test_name = stringify!($test_name);

                    let input = fs::read_to_string(format!("../assets/inputs/{}.corn", test_name)).unwrap();
                    let valid = fs::read_to_string(format!("../assets/outputs/toml/{}.toml", test_name)).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let serialized = toml::to_string_pretty(&config).unwrap().replace("\r", "");

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

                let input = fs::read_to_string(format!("../assets/inputs/{}.corn", test_name)).unwrap();

                let config = parse(input.as_str());
                assert!(config.is_err());
            }
        )+
    }
}

generate_eq_tests!(
    array,
    basic,
    basic_empty_let,
    boolean,
    chained,
    chained_complex,
    char,
    comment,
    compact,
    complex,
    complex_keys,
    environment_variable,
    float,
    input,
    input_references_input,
    integer,
    mixed_array,
    null,
    object,
    object_in_array,
    readme_example,
    spread,
    string,
    string_interpolation,
    value_after_table,
    very_compact
);

generate_invalid_tests!(invalid, invalid_input, invalid_nesting, invalid_spread);
