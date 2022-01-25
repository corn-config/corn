extern crate core;

use cornfig::parse;
use std::fs;

macro_rules! generate_eq_tests {
    ($($test_name:ident),+) => {
        $(
            #[test]
            fn $test_name() {
                let test_name = stringify!($test_name);

                let input = fs::read_to_string(format!("assets/inputs/{}.corn", test_name)).unwrap();
                let output = fs::read_to_string(format!("assets/outputs/{}.json", test_name)).unwrap().replace("\r", "");

                let config = parse(input.as_str()).unwrap();
                let json = serde_json::to_string_pretty(&config.value).unwrap().replace("\r", "");

                assert_eq!(json, output);
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
    variable,
    environment_variable,
    chained,
    chained_complex,
    compact,
    comment,
    complex
);

#[test]
fn invalid() {
    let input = fs::read_to_string("assets/inputs/invalid.corn").unwrap();

    let config = parse(input.as_str());
    assert!(config.is_err());
}
