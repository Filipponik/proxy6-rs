use serde_json::Value;

/// Errors that can be thrown by the API.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Error that exists in the [API documentation](https://px6.me/developers).
    #[error("Documented error occurred: {code:?}, response body: {response}")]
    DocumentedError {
        code: DocumentedErrorCode,
        response: String,
    },

    /// Any `reqwest` error: network error, ssl error, proxy error etc.
    #[error("Reqwest error: {source}")]
    ReqwestError { source: reqwest::Error },

    /// Throttling error. The API is allowed to do no more than 3 queries in 1 second.
    #[error("Too many requests")]
    TooManyRequests { response: String },

    /// API error that is not documented in the [API documentation](https://px6.me/developers).
    #[error("Unknown API error: {response}")]
    UnknownError { response: String },

    /// Success response but cannot parse body to structs.
    #[error("Success response but cannot parse body: {source}, response: {response}")]
    SuccessButCannotParse {
        source: serde_json::Error,
        response: String,
    },
}

/// Error that exists in the [API documentation](https://px6.me/developers).
#[derive(Debug, Clone, thiserror::Error)]
pub enum DocumentedErrorCode {
    /// 30 - Error unknown - Unknown error.
    #[error("Unknown error")]
    Unknown,

    /// 100 - Error key - Authorization error, wrong key.
    #[error("Authorization error, wrong key")]
    Key,

    /// 105 - Error ip - The API was accessed from an incorrect IP (if the restriction is enabled), or an incorrect IP address format.
    #[error(
        "The API was accessed from an incorrect IP (if the restriction is enabled), or an incorrect IP address format"
    )]
    Ip,

    /// 110 - Error method - Wrong method.
    #[error("Wrong method")]
    Method,

    /// 200 - Error count - Wrong proxies quantity, wrong amount or no quantity input.
    #[error("Wrong proxies quantity, wrong amount or no quantity input")]
    Count,

    /// 210 - Error period - Period error, wrong period input (days) or no input.
    #[error("Period error, wrong period input (days) or no input")]
    Period,

    /// 220 - Error country - Country error, wrong country input (iso2 for country input) or no input.
    #[error("Country error, wrong country input (iso2 for country input) or no input")]
    Country,

    /// 230 - Error ids - Error of the list of the proxy numbers. Proxy numbers have to divided with comas.
    #[error("Error of the list of the proxy numbers. Proxy numbers have to divided with comas")]
    Ids,

    /// 240 - Error version - The proxy version is specified incorrectly.
    #[error("The proxy version is specified incorrectly")]
    Version,

    /// 250 - Error descr - Tech description error.
    #[error("Technical description error")]
    Description,

    /// 260 - Error type - Proxy type (protocol) error. Incorrect or missing.
    #[error("Proxy type (protocol) error. Incorrect or missing")]
    Type,

    /// 270 - Error port - Proxy port error, incorrectly specified or missing.
    #[error("Proxy port error, incorrectly specified or missing")]
    Port,

    /// 280 - Error proxy str - Proxy string error for the check method, incorrectly specified.
    #[error("Proxy string error for the check method, incorrectly specified")]
    ProxyString,

    /// 300 - Error active proxy allow - Proxy amount error. Appears after attempt of purchase of more proxies than available on the service.
    #[error(
        "Proxy amount error. Appears after attempt of purchase of more proxies than available on the service"
    )]
    ActiveProxyAllow,

    /// 400 - Error no money - Balance error. Zero or low balance on your account.
    #[error("Balance error. Zero or low balance on your account")]
    NoMoney,

    /// 404 - Error not found - Element error. The requested item was not found.
    #[error("Element error. The requested item was not found")]
    NotFound,

    /// 410 - Error price - Error calculating the cost. The total cost is less than or equal to zero.
    #[error("Error calculating the cost. The total cost is less than or equal to zero")]
    Price,
}

impl DocumentedErrorCode {
    fn from_numeric_code(code: &str) -> Option<Self> {
        Some(match code {
            "30" => Self::Unknown,
            "100" => Self::Key,
            "105" => Self::Ip,
            "110" => Self::Method,
            "200" => Self::Count,
            "210" => Self::Period,
            "220" => Self::Country,
            "230" => Self::Ids,
            "240" => Self::Version,
            "250" => Self::Description,
            "260" => Self::Type,
            "270" => Self::Port,
            "280" => Self::ProxyString,
            "300" => Self::ActiveProxyAllow,
            "400" => Self::NoMoney,
            "404" => Self::NotFound,
            "410" => Self::Price,
            _ => return None,
        })
    }
}

impl ApiError {
    pub(crate) fn parse_from_response_body(body: &str) -> Self {
        let body = body.to_string();

        if let Ok(Value::Object(body_value)) = serde_json::from_str::<Value>(&body)
            && let Some(Value::String(code)) = body_value.get("error_id")
            && let Some(error_code) = DocumentedErrorCode::from_numeric_code(code)
        {
            return Self::DocumentedError {
                code: error_code,
                response: body,
            };
        }

        Self::UnknownError { response: body }
    }
}
