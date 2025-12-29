use serde::Deserializer;
use std::str::FromStr;

fn to_number<'de, D, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::{Deserialize, de::Error, de::Unexpected};
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        #[allow(clippy::option_if_let_else)]
        Value::Number(num) => {
            if let Ok(x) = num.to_string().parse() {
                Ok(x)
            } else {
                Err(Error::invalid_type(
                    Unexpected::Other("number cannot parse to needed number"),
                    &"number or string parsed to number",
                ))
            }
        }
        Value::String(s) => Ok(s.parse().map_err(|_| {
            Error::invalid_type(
                Unexpected::Other("string cannot parse to number"),
                &"number or string parsed to number",
            )
        })?),
        _ => Err(Error::invalid_type(
            Unexpected::Other("non-number/string value"),
            &"number or string parsed to number",
        )),
    }
}

pub fn to_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    to_number(deserializer)
}

pub fn to_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    to_number(deserializer)
}

pub fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    to_number(deserializer)
}

pub fn parse_proxy_status<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::{Deserialize, de::Error, de::Unexpected};
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) => match s.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(Error::invalid_type(
                Unexpected::Other("string must be 0 or 1"),
                &"string 0 or 1",
            )),
        },
        _ => Err(Error::invalid_type(
            Unexpected::Other("non-string value"),
            &"string 0 or 1",
        )),
    }
}

pub fn to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::{Deserialize, de::Error, de::Unexpected};
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Number(num) => Ok(num.to_string()),
        Value::String(s) => Ok(s),
        _ => Err(Error::invalid_type(
            Unexpected::Other("non-number/string value"),
            &"a number or string",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStructU16 {
        #[serde(deserialize_with = "to_u16")]
        value: u16,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStructUsize {
        #[serde(deserialize_with = "to_usize")]
        value: usize,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStructF64 {
        #[serde(deserialize_with = "to_f64")]
        value: f64,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStructString {
        #[serde(deserialize_with = "to_string")]
        value: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStructStatus {
        #[serde(deserialize_with = "parse_proxy_status")]
        status: bool,
    }

    // ===== to_u16 tests =====

    #[test]
    fn test_to_u16_from_number() {
        let json = r#"{"value": 42}"#;
        let result: TestStructU16 = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_to_u16_from_string_valid() {
        let json = r#"{"value": "65535"}"#;
        let result: TestStructU16 = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 65535);
    }

    #[test]
    fn test_to_u16_from_string_zero() {
        let json = r#"{"value": "0"}"#;
        let result: TestStructU16 = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 0);
    }

    #[test]
    fn test_to_u16_from_float_number() {
        let json = r#"{"value": 123.5}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_u16_from_string_invalid() {
        let json = r#"{"value": "not_a_number"}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_u16_from_string_overflow() {
        let json = r#"{"value": "65536"}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_u16_from_negative_number() {
        let json = r#"{"value": -5}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_u16_from_boolean() {
        let json = r#"{"value": true}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_u16_from_null() {
        let json = r#"{"value": null}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_u16_from_array() {
        let json = r#"{"value": [42]}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_u16_from_object() {
        let json = r#"{"value": {}}"#;
        let result: Result<TestStructU16, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ===== to_usize tests =====

    #[test]
    fn test_to_usize_from_number() {
        let json = r#"{"value": 42}"#;
        let result: TestStructUsize = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_to_usize_from_large_number() {
        let json = r#"{"value": 18446744073709551615}"#;
        let result: TestStructUsize = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 18_446_744_073_709_551_615);
    }

    #[test]
    fn test_to_usize_from_string_valid() {
        let json = r#"{"value": "12345"}"#;
        let result: TestStructUsize = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 12345);
    }

    #[test]
    fn test_to_usize_from_string_zero() {
        let json = r#"{"value": "0"}"#;
        let result: TestStructUsize = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 0);
    }

    #[test]
    fn test_to_usize_from_string_invalid() {
        let json = r#"{"value": "not_a_number"}"#;
        let result: Result<TestStructUsize, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_usize_from_string_negative() {
        let json = r#"{"value": "-5"}"#;
        let result: Result<TestStructUsize, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_usize_from_float_number() {
        let json = r#"{"value": 123.5}"#;
        let result: Result<TestStructUsize, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_usize_from_negative_number() {
        let json = r#"{"value": -5}"#;
        let result: Result<TestStructUsize, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_usize_from_boolean() {
        let json = r#"{"value": true}"#;
        let result: Result<TestStructUsize, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_usize_from_null() {
        let json = r#"{"value": null}"#;
        let result: Result<TestStructUsize, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_usize_from_array() {
        let json = r#"{"value": [42]}"#;
        let result: Result<TestStructUsize, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ===== to_f64 tests =====

    #[test]
    fn test_to_f64_from_integer_number() {
        let json = r#"{"value": 42}"#;
        let result: TestStructF64 = serde_json::from_str(json).unwrap();
        assert!((result.value - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_f64_from_float_number() {
        let json = r#"{"value": 123.456}"#;
        let result: TestStructF64 = serde_json::from_str(json).unwrap();
        assert!((result.value - 123.456).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_f64_from_negative_float() {
        let json = r#"{"value": -123.456}"#;
        let result: TestStructF64 = serde_json::from_str(json).unwrap();
        assert!((result.value - (-123.456)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_f64_from_string_integer() {
        let json = r#"{"value": "42"}"#;
        let result: TestStructF64 = serde_json::from_str(json).unwrap();
        assert!((result.value - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_f64_from_string_float() {
        let json = r#"{"value": "123.456"}"#;
        let result: TestStructF64 = serde_json::from_str(json).unwrap();
        assert!((result.value - 123.456).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_f64_from_scientific_notation_number() {
        let json = r#"{"value": 1.5e10}"#;
        let result: TestStructF64 = serde_json::from_str(json).unwrap();
        assert!((result.value - 15_000_000_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_f64_from_string_scientific() {
        let json = r#"{"value": "1.5e10"}"#;
        let result: TestStructF64 = serde_json::from_str(json).unwrap();
        assert!((result.value - 15_000_000_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_f64_from_string_invalid() {
        let json = r#"{"value": "not_a_number"}"#;
        let result: Result<TestStructF64, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_f64_from_boolean() {
        let json = r#"{"value": true}"#;
        let result: Result<TestStructF64, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_f64_from_null() {
        let json = r#"{"value": null}"#;
        let result: Result<TestStructF64, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_f64_from_array() {
        let json = r#"{"value": [42.0]}"#;
        let result: Result<TestStructF64, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ===== to_string tests =====

    #[test]
    fn test_to_string_from_number_integer() {
        let json = r#"{"value": 42}"#;
        let result: TestStructString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "42");
    }

    #[test]
    fn test_to_string_from_number_float() {
        let json = r#"{"value": 123.456}"#;
        let result: TestStructString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "123.456");
    }

    #[test]
    fn test_to_string_from_string() {
        let json = r#"{"value": "hello"}"#;
        let result: TestStructString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "hello");
    }

    #[test]
    fn test_to_string_from_empty_string() {
        let json = r#"{"value": ""}"#;
        let result: TestStructString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "");
    }

    #[test]
    fn test_to_string_from_zero() {
        let json = r#"{"value": 0}"#;
        let result: TestStructString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "0");
    }

    #[test]
    fn test_to_string_from_negative_number() {
        let json = r#"{"value": -42}"#;
        let result: TestStructString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "-42");
    }

    #[test]
    fn test_to_string_from_boolean() {
        let json = r#"{"value": true}"#;
        let result: Result<TestStructString, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_string_from_null() {
        let json = r#"{"value": null}"#;
        let result: Result<TestStructString, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_string_from_array() {
        let json = r#"{"value": [42]}"#;
        let result: Result<TestStructString, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_string_from_object() {
        let json = r#"{"value": {}}"#;
        let result: Result<TestStructString, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ===== parse_proxy_status tests =====

    #[test]
    fn test_parse_proxy_status_from_string_zero() {
        let json = r#"{"status": "0"}"#;
        let result: TestStructStatus = serde_json::from_str(json).unwrap();
        assert!(!result.status);
    }

    #[test]
    fn test_parse_proxy_status_from_string_one() {
        let json = r#"{"status": "1"}"#;
        let result: TestStructStatus = serde_json::from_str(json).unwrap();
        assert!(result.status);
    }

    #[test]
    fn test_parse_proxy_status_from_string_invalid() {
        let json = r#"{"status": "2"}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_string_text() {
        let json = r#"{"status": "true"}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_empty_string() {
        let json = r#"{"status": ""}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_boolean_true() {
        let json = r#"{"status": true}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_boolean_false() {
        let json = r#"{"status": false}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_number_zero() {
        let json = r#"{"status": 0}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_number_one() {
        let json = r#"{"status": 1}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_null() {
        let json = r#"{"status": null}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_proxy_status_from_array() {
        let json = r#"{"status": ["1"]}"#;
        let result: Result<TestStructStatus, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
