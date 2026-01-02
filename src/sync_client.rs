use crate::{
    ApiResult, ClientBuildError, error,
    method::{self, ApiMethod},
    params, response,
};

const DEFAULT_BASE_URL: &str = "https://px6.link";

#[derive(Debug, Clone)]
pub struct SyncClient {
    base_url: String,
    requester: reqwest::blocking::Client,
    api_key: String,
}

#[derive(Default, Debug, Clone)]
pub struct SyncClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    requester: Option<reqwest::blocking::Client>,
}

impl SyncClientBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    #[must_use]
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    #[must_use]
    pub fn requester(mut self, requester: reqwest::blocking::Client) -> Self {
        self.requester = Some(requester);
        self
    }

    /// Builds a new client.
    ///
    /// # Errors
    /// - [`ClientBuildError::ApiKeyMustBeSet`] if the API key is not set.
    pub fn build(self) -> Result<SyncClient, ClientBuildError> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let api_key = self.api_key.ok_or(ClientBuildError::ApiKeyMustBeSet)?;
        let requester = self.requester.unwrap_or_default();

        Ok(SyncClient {
            base_url,
            requester,
            api_key,
        })
    }
}

impl SyncClient {
    #[must_use]
    pub fn builder() -> SyncClientBuilder {
        SyncClientBuilder::new()
    }

    fn get_request_with_params<TResponse: serde::de::DeserializeOwned>(
        &self,
        method: &method::ApiMethod,
    ) -> Result<TResponse, error::ApiError> {
        let url = format!(
            "{}/api/{}/{}?{}",
            self.base_url,
            self.api_key,
            method,
            method.get_params().to_query_string()
        );

        let response = self
            .requester
            .get(url)
            .send()
            .map_err(|err| error::ApiError::ReqwestError { source: err })?;

        let response_status = response.status();
        let response_text = response
            .text()
            .map_err(|err| error::ApiError::ReqwestError { source: err })?;

        if response_status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(error::ApiError::TooManyRequests {
                response: response_text,
            });
        }

        if let Some(possible_error) =
            error::DocumentedErrorCode::parse_from_response_body(&response_text)
        {
            return Err(error::ApiError::DocumentedError {
                response: response_text,
                code: possible_error,
            });
        }

        if !response_status.is_success() {
            return Err(error::ApiError::UnknownError {
                response: response_text,
            });
        }

        serde_json::from_str(&response_text).map_err(|err| error::ApiError::SuccessButCannotParse {
            source: err,
            response: response_text,
        })
    }

    /// Get information about the cost of the order, depending on the version, period and number of proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn get_price(&self, params: params::GetPrice) -> ApiResult<response::GetPrice> {
        self.get_request_with_params(&ApiMethod::GetPrice(params))
    }

    /// Get information on amount of proxies available to purchase for a selected country.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn get_count(&self, params: params::GetCount) -> ApiResult<response::GetCount> {
        self.get_request_with_params(&ApiMethod::GetCount(params))
    }

    /// Get information on available for proxies purchase countries.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn get_country(&self, params: params::GetCountry) -> ApiResult<response::GetCountry> {
        self.get_request_with_params(&ApiMethod::GetCountry(params))
    }

    /// Get the list of your proxies.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn get_proxy(&self, params: params::GetProxy) -> ApiResult<response::GetProxy> {
        self.get_request_with_params(&ApiMethod::GetProxy(params))
    }

    /// Change the type (protocol) of your proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    ///
    /// Note that if all proxies in which you want to change the type already have the appropriate type (protocol), it will return an [`error::ApiError::DocumentedError`] with code [`error::DocumentedErrorCode::Unknown`].
    pub fn set_type(&self, params: params::SetType) -> ApiResult<response::SuccessResponse> {
        self.get_request_with_params(&ApiMethod::SetType(params))
    }

    /// Update technical comments in the proxy list that was added when buying.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn set_description(
        &self,
        params: params::SetDescription,
    ) -> ApiResult<response::SetDescription> {
        self.get_request_with_params(&ApiMethod::SetDescription(params))
    }

    /// Purchase proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn buy(&self, params: params::Buy) -> ApiResult<response::Buy> {
        self.get_request_with_params(&ApiMethod::Buy(params))
    }

    /// Extend existing proxies.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn prolong(&self, params: params::Prolong) -> ApiResult<response::Prolong> {
        self.get_request_with_params(&ApiMethod::Prolong(params))
    }

    /// Delete existing proxies.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn delete(&self, params: params::Delete) -> ApiResult<response::Delete> {
        self.get_request_with_params(&ApiMethod::Delete(params))
    }

    /// Check the validity of the proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn check(&self, params: params::Check) -> ApiResult<response::Check> {
        self.get_request_with_params(&ApiMethod::Check(params))
    }

    /// Attach or detach IP address auth from the proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub fn ip_auth(&self, params: params::IpAuth) -> ApiResult<response::SuccessResponse> {
        self.get_request_with_params(&ApiMethod::IpAuth(params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        let builder = SyncClientBuilder::new();
        assert_eq!(builder.base_url, None);
        assert_eq!(builder.api_key, None);
        assert!(builder.requester.is_none());
    }

    #[test]
    fn test_builder_base_url() {
        let builder = SyncClientBuilder::new().base_url("https://custom.example.com");
        assert_eq!(
            builder.base_url,
            Some("https://custom.example.com".to_string())
        );
        assert_eq!(builder.api_key, None);
        assert!(builder.requester.is_none());
    }

    #[test]
    fn test_builder_api_key() {
        let builder = SyncClientBuilder::new().api_key("test-api-key");
        assert_eq!(builder.base_url, None);
        assert_eq!(builder.api_key, Some("test-api-key".to_string()));
        assert!(builder.requester.is_none());
    }

    #[test]
    fn test_builder_requester() {
        let requester = reqwest::blocking::Client::new();
        let builder = SyncClientBuilder::new().requester(requester);
        assert_eq!(builder.base_url, None);
        assert_eq!(builder.api_key, None);
        assert!(builder.requester.is_some());
    }

    #[test]
    fn test_builder_build_success() {
        let client = SyncClientBuilder::new()
            .base_url("https://custom.example.com")
            .api_key("test-api-key")
            .build()
            .unwrap();

        assert_eq!(client.base_url, "https://custom.example.com");
        assert_eq!(client.api_key, "test-api-key");
        // Client always has a requester
    }

    #[test]
    fn test_builder_build_with_default_base_url() {
        let client = SyncClientBuilder::new()
            .api_key("test-api-key")
            .build()
            .unwrap();

        assert_eq!(client.base_url, DEFAULT_BASE_URL);
        assert_eq!(client.api_key, "test-api-key");
    }

    #[test]
    fn test_builder_build_without_api_key_error() {
        let result = SyncClientBuilder::new().build();
        assert!(matches!(result, Err(ClientBuildError::ApiKeyMustBeSet)));
    }

    #[test]
    fn test_client_builder() {
        let client = SyncClient::builder();
        assert_eq!(client.base_url, None);
        assert_eq!(client.api_key, None);
        assert!(client.requester.is_none());
    }
}
