use serde_json::Value;

/// Errors that can be thrown by the API.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Error that exists in the [API documentation](https://px6.me/developers).
    #[error("Documented error occurred: {code}, response body: {response}")]
    DocumentedError {
        code: DocumentedErrorCode,
        response: String,
    },

    /// Any `reqwest` error: network error, ssl error, proxy error etc.
    #[error("Reqwest error: {source}")]
    ReqwestError { source: reqwest::Error },

    /// Throttling error. The API is allowed to do no more than 3 queries in 1 second.
    #[error("Too many requests: {response}")]
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
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
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
    const fn from_numeric_code(code: usize) -> Option<Self> {
        Some(match code {
            30 => Self::Unknown,
            100 => Self::Key,
            105 => Self::Ip,
            110 => Self::Method,
            200 => Self::Count,
            210 => Self::Period,
            220 => Self::Country,
            230 => Self::Ids,
            240 => Self::Version,
            250 => Self::Description,
            260 => Self::Type,
            270 => Self::Port,
            280 => Self::ProxyString,
            300 => Self::ActiveProxyAllow,
            400 => Self::NoMoney,
            404 => Self::NotFound,
            410 => Self::Price,
            _ => return None,
        })
    }

    pub(crate) fn parse_from_response_body(body: &str) -> Option<Self> {
        if let Ok(Value::Object(body_value)) = serde_json::from_str::<Value>(body)
            && let Some(Value::Number(code)) = body_value.get("error_id")
            && let Some(Ok(code)) = code.as_u64().map(usize::try_from) // cast Number to usize
            && let Some(code) = Self::from_numeric_code(code)
        {
            return Some(code);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_documented_error_code_from_numeric_code() {
        // Test all known error codes
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(30),
            Some(DocumentedErrorCode::Unknown)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(100),
            Some(DocumentedErrorCode::Key)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(105),
            Some(DocumentedErrorCode::Ip)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(110),
            Some(DocumentedErrorCode::Method)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(200),
            Some(DocumentedErrorCode::Count)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(210),
            Some(DocumentedErrorCode::Period)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(220),
            Some(DocumentedErrorCode::Country)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(230),
            Some(DocumentedErrorCode::Ids)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(240),
            Some(DocumentedErrorCode::Version)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(250),
            Some(DocumentedErrorCode::Description)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(260),
            Some(DocumentedErrorCode::Type)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(270),
            Some(DocumentedErrorCode::Port)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(280),
            Some(DocumentedErrorCode::ProxyString)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(300),
            Some(DocumentedErrorCode::ActiveProxyAllow)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(400),
            Some(DocumentedErrorCode::NoMoney)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(404),
            Some(DocumentedErrorCode::NotFound)
        );
        assert_eq!(
            DocumentedErrorCode::from_numeric_code(410),
            Some(DocumentedErrorCode::Price)
        );

        // Test unknown error codes
        assert_eq!(DocumentedErrorCode::from_numeric_code(0), None);
        assert_eq!(DocumentedErrorCode::from_numeric_code(999), None);
        assert_eq!(DocumentedErrorCode::from_numeric_code(50), None);
    }

    #[test]
    fn test_parse_from_response_body_valid() {
        // Test valid JSON with error_id field
        let body = r#"{"error_id": 100, "message": "Authorization error"}"#;
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(body),
            Some(DocumentedErrorCode::Key)
        );

        let body = r#"{"error_id": 200, "error": "Wrong count"}"#;
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(body),
            Some(DocumentedErrorCode::Count)
        );

        let body = r#"{"error_id": 400, "balance": 0}"#;
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(body),
            Some(DocumentedErrorCode::NoMoney)
        );

        let body = r#"{"error_id": 30}"#;
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(body),
            Some(DocumentedErrorCode::Unknown)
        );
    }

    #[test]
    fn test_parse_from_response_body_invalid() {
        // Test invalid JSON
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body("invalid json"),
            None
        );

        // Test JSON without error_id field
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(r#"{"message": "error"}"#),
            None
        );

        // Test JSON with non-numeric error_id
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(r#"{"error_id": "string"}"#),
            None
        );

        // Test JSON with unknown error_id
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(r#"{"error_id": 999}"#),
            None
        );

        // Test empty string
        assert_eq!(DocumentedErrorCode::parse_from_response_body(""), None);

        // Test JSON array instead of object
        assert_eq!(
            DocumentedErrorCode::parse_from_response_body(r#"[{"error_id": 100}]"#),
            None
        );
    }

    #[test]
    fn test_api_error_display() {
        // Test DocumentedError display
        let err = ApiError::DocumentedError {
            code: DocumentedErrorCode::Key,
            response: "Authorization failed".to_string(),
        };
        assert!(err.to_string().contains("Authorization error, wrong key"));
        assert!(err.to_string().contains("Authorization failed"));

        // Test DocumentedError with different code
        let err = ApiError::DocumentedError {
            code: DocumentedErrorCode::NoMoney,
            response: "Low balance".to_string(),
        };
        assert!(
            err.to_string()
                .contains("Balance error. Zero or low balance on your account")
        );
        assert!(err.to_string().contains("Low balance"));

        // Test UnknownError display
        let err = ApiError::UnknownError {
            response: "Something went wrong".to_string(),
        };
        assert!(err.to_string().contains("Unknown API error"));
        assert!(err.to_string().contains("Something went wrong"));

        // Test TooManyRequests display
        let err = ApiError::TooManyRequests {
            response: "Rate limit exceeded".to_string(),
        };
        assert!(err.to_string().contains("Too many requests"));
        assert!(err.to_string().contains("Rate limit exceeded"));

        // Test SuccessButCannotParse display
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err = ApiError::SuccessButCannotParse {
            source: json_err,
            response: "invalid response".to_string(),
        };
        assert!(
            err.to_string()
                .contains("Success response but cannot parse body")
        );
        assert!(err.to_string().contains("invalid response"));
    }

    #[test]
    fn test_documented_error_code_display() {
        // Test error messages for all documented error codes
        assert_eq!(format!("{}", DocumentedErrorCode::Unknown), "Unknown error");
        assert_eq!(
            format!("{}", DocumentedErrorCode::Key),
            "Authorization error, wrong key"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Ip),
            "The API was accessed from an incorrect IP (if the restriction is enabled), or an incorrect IP address format"
        );
        assert_eq!(format!("{}", DocumentedErrorCode::Method), "Wrong method");
        assert_eq!(
            format!("{}", DocumentedErrorCode::Count),
            "Wrong proxies quantity, wrong amount or no quantity input"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Period),
            "Period error, wrong period input (days) or no input"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Country),
            "Country error, wrong country input (iso2 for country input) or no input"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Ids),
            "Error of the list of the proxy numbers. Proxy numbers have to divided with comas"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Version),
            "The proxy version is specified incorrectly"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Description),
            "Technical description error"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Type),
            "Proxy type (protocol) error. Incorrect or missing"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Port),
            "Proxy port error, incorrectly specified or missing"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::ProxyString),
            "Proxy string error for the check method, incorrectly specified"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::ActiveProxyAllow),
            "Proxy amount error. Appears after attempt of purchase of more proxies than available on the service"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::NoMoney),
            "Balance error. Zero or low balance on your account"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::NotFound),
            "Element error. The requested item was not found"
        );
        assert_eq!(
            format!("{}", DocumentedErrorCode::Price),
            "Error calculating the cost. The total cost is less than or equal to zero"
        );
    }

    #[test]
    fn test_documented_error_code_clone() {
        let err1 = DocumentedErrorCode::Key;
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
