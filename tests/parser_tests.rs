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
                    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

                    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/{test_name}.corn")).unwrap();
                    let valid = fs::read_to_string(format!("{root_dir}/assets/outputs/json/{test_name}.json")).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let serialized = serde_json::to_string_pretty(&config).unwrap().replace("\r", "");

                    assert_eq!(serialized.trim(), valid.trim());
                }

                #[test]
                fn [<yaml_ $test_name>]() {
                    let test_name = stringify!($test_name);
                    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

                    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/{test_name}.corn")).unwrap();
                    let valid = fs::read_to_string(format!("{root_dir}/assets/outputs/yaml/{test_name}.yml")).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    let serialized = serde_norway::to_string(&config).unwrap().replace("\r", "");

                    assert_eq!(serialized.trim(), valid.trim());
                }

                #[test]
                fn [<toml_ $test_name>]() {
                    let test_name = stringify!($test_name);
                    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

                    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/{test_name}.corn")).unwrap();
                    let valid = fs::read_to_string(format!("{root_dir}/assets/outputs/toml/{test_name}.toml")).unwrap().replace("\r", "");

                    let config = parse(input.as_str()).unwrap();
                    // fall back to default as toml can fail due to no null
                    let serialized = toml_edit::ser::to_string_pretty(&config).unwrap_or_default().replace("\r", "");

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
                let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

                let input = fs::read_to_string(format!("{root_dir}/assets/inputs/{}.corn", test_name)).unwrap();

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
    null_in_array,
    object,
    object_in_array,
    quoted_keys,
    readme_example,
    spread,
    string,
    string_interpolation,
    value_after_table,
    very_compact
);

generate_invalid_tests!(invalid, invalid_input, invalid_nesting, invalid_spread);
