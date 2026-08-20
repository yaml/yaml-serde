use serde_derive::Deserialize;
use yaml_serde::{from_value, Value};

#[test]
fn test_number_to_string_coercion() {
    let v = Value::Number(123.into());
    let s: String = from_value(v).expect("Failed to coerce number to string");
    assert_eq!(s, "123");
}

#[test]
fn test_bool_to_string_coercion() {
    let v = Value::Bool(true);
    let s: String = from_value(v).expect("Failed to coerce bool to string");
    assert_eq!(s, "true");
}

#[test]
fn test_number_as_identifier() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Data {
        #[serde(rename = "123")]
        field: String,
    }
    
    let mut map = yaml_serde::Mapping::new();
    map.insert(Value::Number(123.into()), Value::String("value".into()));
    let v = Value::Mapping(map);
    
    let data: Data = from_value(v).expect("Failed to use number as identifier");
    assert_eq!(data.field, "value");
}
