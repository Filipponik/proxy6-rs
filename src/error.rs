use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Documented error occurred: {code:?}, response body: {response}")]
    DocumentedError {
        code: DocumentedErrorCode,
        response: String,
    },
    #[error("Reqwest error: {source}")]
    ReqwestError { source: reqwest::Error },
    #[error("Too many requests")]
    TooManyRequests { response: String },
    #[error("Unknown API error: {response}")]
    UnknownError { response: String },
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum DocumentedErrorCode {
    #[error("Unknown error")]
    Unknown,
    #[error("Authorization error, wrong key")]
    Key,
    #[error(
        "The API was accessed from an incorrect IP (if the restriction is enabled), or an incorrect IP address format"
    )]
    Ip,
    #[error("Wrong method")]
    Method,
    #[error("Wrong proxies quantity, wrong amount or no quantity input")]
    Count,
    #[error("Period error, wrong period input (days) or no input")]
    Period,
    #[error("Country error, wrong country input (iso2 for country input) or no input")]
    Country,
    #[error("Error of the list of the proxy numbers. Proxy numbers have to divided with comas")]
    Ids,
    #[error("The proxy version is specified incorrectly")]
    Version,
    #[error("Technical description error")]
    Description,
    #[error("Proxy type (protocol) error. Incorrect or missing")]
    Type,
    #[error("Proxy port error, incorrectly specified or missing")]
    Port,
    #[error("Proxy string error for the check method, incorrectly specified")]
    ProxyString,
    #[error(
        "Proxy amount error. Appears after attempt of purchase of more proxies than available on the service"
    )]
    ActiveProxyAllow,
    #[error("Balance error. Zero or low balance on your account")]
    NoMoney,
    #[error("Element error. The requested item was not found")]
    NotFound,
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
        let Ok(Value::Object(body_value)) = serde_json::from_str::<Value>(&body) else {
            return Self::UnknownError { response: body };
        };

        if let Some(Value::String(code)) = body_value.get("error_id")
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
