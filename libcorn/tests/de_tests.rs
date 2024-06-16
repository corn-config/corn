use corn::from_str;
use paste::paste;
use serde::Deserialize;
use std::fs;

macro_rules! generate_eq_tests {
    ($(($test_name:ident, $test_type:ty)),+) => {
        $(
            paste! {
                #[test]
                fn $test_name() {
                    let test_name = stringify!($test_name);

                    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
                    let config = from_str::<$test_type>(&input).unwrap();

                    let json_input = fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
                    let json_config = serde_json::from_str(&json_input).unwrap();

                    assert_eq!(config, json_config);
                }
            }
        )+
    };
}

#[derive(Deserialize, Debug, PartialEq)]
struct Empty {}

#[derive(Deserialize, Debug, PartialEq)]
struct Array {
    foo: Vec<i64>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Basic {
    foo: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum BasicNewTypeEnum {
    Foo(String),
}

#[derive(Deserialize, Debug, PartialEq)]
struct BasicUnitEnum {
    foo: BasicUnitEnumInner,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum BasicUnitEnumInner {
    Bar,
}

#[derive(Deserialize, Debug, PartialEq)]
struct BasicNewType {
    foo: BasicNewTypeInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct BasicNewTypeInner(String);

#[derive(Deserialize, Debug, PartialEq)]
struct Boolean {
    foo: bool,
    bar: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Bytes {
    #[serde(with = "serde_bytes")]
    foo: Vec<u8>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Chained {
    foo: ChainedInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedInner {
    bar: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ChainedEnum {
    Foo { bar: String },
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedComplex {
    foo: ChainedComplexInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedComplexInner {
    bar: ChainedComplexInnerInner,
    qux: bool,
    quux: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedComplexInnerInner {
    baz: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Char {
    foo: char,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Compact {
    one: CompactOne,
    two: CompactTwo,
    three: CompactThree,
    four: CompactFour,
    five: CompactFive,
    six: CompactSix,
    seven: CompactSeven,
    eight: Vec<String>,
    nine: Vec<bool>,
    ten: (u8, u8),
    eleven: Vec<Vec<u8>>,
    twelve: Vec<Empty>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactOne {
    foo: String,
    bar: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactTwo {
    foo: i64,
    bar: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactThree {
    foo: f64,
    bar: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactFour {
    foo: bool,
    bar: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactFive {
    foo: (),
    bar: (),
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactSix {
    foo: Empty,
    bar: Empty,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactSeven {
    foo: Vec<u8>,
    bar: Vec<u8>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Complex {
    age: i64,
    employment: ComplexEmployment,
    empty1: Empty,
    empty2: Vec<u8>,
    favourites: (String, String, String, String, f64, bool, Favourites),
    gender: String,
    name: ComplexName,
    negative: ComplexNegative,
    parents: ComplexParents,
    placeholder: (),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ComplexEmployment {
    employed: bool,
    name: String,
    since_year: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Favourites {
    food: ComplexFavouritesFood,
    hello: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexFavouritesFood {
    favourite: String,
    hated: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexName {
    first: String,
    full: String,
    last: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexNegative {
    float: f64,
    int: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexParents {
    father: ComplexParentsFather,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexParentsFather {
    birthday: ComplexParentsFatherBirthday,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexParentsFatherBirthday {
    day: i64,
    month: i64,
    year: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexKeys {
    #[serde(rename = "!\"£$%^&*()_")]
    symbols: i64,
    #[serde(rename = "apple-pie")]
    apple_pie: ComplexKeysApplePie,
    foo: ComplexKeysFoo,
    j12345: i64,
    #[serde(rename = "with-dash")]
    with_dash: i64,
    #[serde(rename = "with_underscore")]
    with_underscore: i64,
    #[serde(rename = "with_🌽")]
    with_corn_emoji: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexKeysApplePie {
    crust: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexKeysFoo {
    #[serde(rename = "bar-baz")]
    bar_baz: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Float {
    foo: f64,
    bar: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Input {
    name: InputName,
    dob: InputDob,
}

#[derive(Deserialize, Debug, PartialEq)]
struct InputName {
    first: String,
    last: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct InputDob {
    day: u8,
    month: u8,
    year: u16,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Integer {
    foo: i64,
    bar: i64,
    baz: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct MixedArray {
    foo: (u8, String, bool),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum MixedArrayEnum {
    Foo(u8, String, bool),
}

#[derive(Deserialize, Debug, PartialEq)]
struct Null {
    foo: (),
}

#[derive(Deserialize, Debug, PartialEq)]
struct NullOption {
    foo: Option<u8>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct NullUnit {
    foo: NullUnitInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct NullUnitInner;

#[derive(Deserialize, Debug, PartialEq)]
struct NullInArray {
    foo: Vec<Option<u8>>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Object {
    foo: SubObject,
}

#[derive(Deserialize, Debug, PartialEq)]
struct SubObject {
    bar: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ObjectInArray {
    foo: Vec<ObjectInArrayFoo>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ObjectInArrayFoo {
    bar: i64,
    foo: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ReadmeExample {
    author: ReadmeExampleAuthor,
    bin: ReadmeExampleBin,
    config: ReadmeExampleConfig,
    contributors: Vec<ReadmeExampleContributor>,
    dependencies: ReadmeExampleDependencies,
    dev_dependencies: ReadmeExampleDevDependencies,
    main: String,
    name: String,
    private: bool,
    scripts: ReadmeExampleScripts,
    version: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct QuotedKeys {
    #[serde(rename = "foo.bar")]
    foo_bar: u8,
    #[serde(rename = "green.eggs")]
    green_eggs: GreenEggs,
    #[serde(rename = "with spaces")]
    with_spaces: bool,
    #[serde(rename = "escaped'quote")]
    escaped_quote: bool,
    #[serde(rename = "escaped=equals")]
    escaped_equals: i8,
}

#[derive(Deserialize, Debug, PartialEq)]
struct GreenEggs {
    and: And,
}

#[derive(Deserialize, Debug, PartialEq)]
struct And {
    ham: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct String_ {
    foo: String,
    bar: String,
    baz: String,
    qux: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleAuthor {
    email: String,
    name: String,
    url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleBin {
    filebrowser: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleConfig {
    hostname: Option<String>,
    port: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleContributor {
    email: String,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleDependencies {
    dotenv: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleDevDependencies {
    typescript: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleScripts {
    build: String,
    run: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ValueAfterTable {
    foo: Empty,
    qux: bool,
}

generate_eq_tests!(
    (array, Array),
    (basic, Basic),
    (basic_empty_let, Basic),
    (boolean, Boolean),
    (chained, Chained),
    (chained_complex, ChainedComplex),
    (char, Char),
    (comment, Basic),
    (compact, Compact),
    (complex, Complex),
    (complex_keys, ComplexKeys),
    (environment_variable, Basic),
    (float, Float),
    (input, Input),
    (input_references_input, Basic),
    (integer, Integer),
    (mixed_array, MixedArray),
    (null, Null),
    (null_in_array, NullInArray),
    (object, Object),
    (object_in_array, ObjectInArray),
    (readme_example, ReadmeExample),
    (quoted_keys, QuotedKeys),
    (string, String_),
    (string_interpolation, Basic),
    (value_after_table, ValueAfterTable),
    (very_compact, Compact)
);

// TODO: Several of these can use the macro, tidy

#[test]
fn basic_new_type_enum() {
    let test_name = "basic";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<BasicNewTypeEnum>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}

#[test]
fn basic_unit_enum() {
    let test_name = "basic";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<BasicUnitEnum>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}

#[test]
fn basic_new_type() {
    let test_name = "basic";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<BasicNewType>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}

#[test]
fn bytes() {
    let test_name = "basic";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<Bytes>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}

#[test]
fn chained_enum() {
    let test_name = "chained";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<ChainedEnum>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}

#[test]
fn mixed_array_enum() {
    let test_name = "mixed_array";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<MixedArrayEnum>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}

#[test]
fn null_option() {
    let test_name = "null";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<NullOption>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}

#[test]
fn null_unit() {
    let test_name = "null";

    let input = fs::read_to_string(format!("../assets/inputs/{test_name}.corn")).unwrap();
    let config = from_str::<NullUnit>(&input).unwrap();

    let json_input =
        fs::read_to_string(format!("../assets/outputs/json/{test_name}.json")).unwrap();
    let json_config = serde_json::from_str(&json_input).unwrap();

    assert_eq!(config, json_config);
}
