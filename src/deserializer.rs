use serde::Deserializer;

pub fn to_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::{Deserialize, de::Error, de::Unexpected};
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        #[allow(clippy::option_if_let_else)]
        Value::Number(num) => {
            if let Some(x) = num.as_u64() {
                u16::try_from(x).map_err(|_| Error::custom("number cannot parse to u16"))
            } else {
                Err(Error::invalid_type(
                    Unexpected::Other("number cannot parse to u16"),
                    &"number or string parsed to u16",
                ))
            }
        }
        Value::String(s) => Ok(s.parse().map_err(|_| {
            Error::invalid_type(
                Unexpected::Other("string cannot parse to u16"),
                &"number or string parsed to u16",
            )
        })?),
        _ => Err(Error::invalid_type(
            Unexpected::Other("non-number/string value"),
            &"number or string parsed to u16",
        )),
    }
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
