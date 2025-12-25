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
