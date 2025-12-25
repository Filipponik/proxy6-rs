use crate::method::ApiMethod;
pub use value_object::*;

const DEFAULT_BASE_URL: &str = "https://px6.link";

pub(crate) mod deserializer;
pub mod error;
mod method;
pub mod params;
pub mod response;
mod value_object;

#[derive(Debug, thiserror::Error)]
pub enum ClientBuildError {
    #[error("API key must be set")]
    ApiKeyMustBeSet,
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    requester: reqwest::Client,
    api_key: String,
}

#[derive(Default, Debug, Clone)]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    requester: Option<reqwest::Client>,
}

type ApiResult<T> = Result<T, error::ApiError>;

impl ClientBuilder {
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
    pub fn requester(mut self, requester: reqwest::Client) -> Self {
        self.requester = Some(requester);
        self
    }

    /// Builds a new client.
    ///
    /// # Errors
    /// - [`ClientBuildError::ApiKeyMustBeSet`] if the API key is not set.
    pub fn build(self) -> Result<Client, ClientBuildError> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let api_key = self.api_key.ok_or(ClientBuildError::ApiKeyMustBeSet)?;
        let requester = self.requester.unwrap_or_default();

        Ok(Client {
            base_url,
            requester,
            api_key,
        })
    }
}

impl Client {
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    async fn get_request_with_params<TResponse: serde::de::DeserializeOwned>(
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
            .await
            .map_err(|err| error::ApiError::ReqwestError { source: err })?;

        let response_status = response.status();
        let response_text = response
            .text()
            .await
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
        };

        if !response_status.is_success() {
            return Err(error::ApiError::UnknownError {
                response: response_text,
            })?;
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
    pub async fn get_price(&self, params: params::GetPrice) -> ApiResult<response::GetPrice> {
        self.get_request_with_params(&ApiMethod::GetPrice(params))
            .await
    }

    /// Get information on amount of proxies available to purchase for a selected country.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn get_count(&self, params: params::GetCount) -> ApiResult<response::GetCount> {
        self.get_request_with_params(&ApiMethod::GetCount(params))
            .await
    }

    /// Get information on available for proxies purchase countries.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn get_country(&self, params: params::GetCountry) -> ApiResult<response::GetCountry> {
        self.get_request_with_params(&ApiMethod::GetCountry(params))
            .await
    }

    /// Get the list of your proxies.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn get_proxy(&self, params: params::GetProxy) -> ApiResult<response::GetProxy> {
        self.get_request_with_params(&ApiMethod::GetProxy(params))
            .await
    }

    /// Change the type (protocol) of your proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    ///
    /// Note that if all proxies in which you want to change the type already have the appropriate type (protocol), it will return an [`error::ApiError::DocumentedError`] with code [`error::DocumentedErrorCode::Unknown`].
    pub async fn set_type(&self, params: params::SetType) -> ApiResult<response::SuccessResponse> {
        self.get_request_with_params(&ApiMethod::SetType(params))
            .await
    }

    /// Update technical comments in the proxy list that was added when buying.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn set_description(
        &self,
        params: params::SetDescription,
    ) -> ApiResult<response::SetDescription> {
        self.get_request_with_params(&ApiMethod::SetDescription(params))
            .await
    }

    /// Purchase proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn buy(&self, params: params::Buy) -> ApiResult<response::Buy> {
        self.get_request_with_params(&ApiMethod::Buy(params)).await
    }

    /// Extend existing proxies.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn prolong(&self, params: params::Prolong) -> ApiResult<response::Prolong> {
        self.get_request_with_params(&ApiMethod::Prolong(params))
            .await
    }

    /// Delete existing proxies.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn delete(&self, params: params::Delete) -> ApiResult<response::Delete> {
        self.get_request_with_params(&ApiMethod::Delete(params))
            .await
    }

    /// Check the validity of the proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn check(&self, params: params::Check) -> ApiResult<response::Check> {
        self.get_request_with_params(&ApiMethod::Check(params))
            .await
    }

    /// Attach or detach IP address auth from the proxy.
    ///
    /// # Errors
    /// Any error can be thrown (see [`error::ApiError`])
    pub async fn ip_auth(&self, params: params::IpAuth) -> ApiResult<response::SuccessResponse> {
        self.get_request_with_params(&ApiMethod::IpAuth(params))
            .await
    }
}
