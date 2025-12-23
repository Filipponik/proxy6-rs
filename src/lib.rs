#![allow(dead_code, unused)]

use reqwest::{Error, Response};

use crate::method::ApiMethod;

const DEFAULT_BASE_URL: &str = "https://px6.link";

pub mod error;
mod method;
mod params;
mod response;

#[derive(Debug, thiserror::Error)]
pub enum ClientBuildError {
    #[error("API key must be set")]
    ApiKeyMustBeSet,
}

#[derive(Debug, Clone)]
struct Client {
    base_url: String,
    requester: reqwest::Client,
    api_key: String,
}

#[derive(Default, Debug, Clone)]
struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    requester: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn requester(mut self, requester: reqwest::Client) -> Self {
        self.requester = Some(requester);
        self
    }

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
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    async fn get_request_with_params<TResponse>(
        &self,
        method: method::ApiMethod,
    ) -> Result<TResponse, error::ApiError> {
        let url = format!(
            "{}/api/{}/{}?{}",
            self.base_url,
            self.api_key,
            method.to_string(),
            method.get_params().to_query_string()
        );

        let response = self.requester.get(url).send().await;

        match response {
            Ok(value) => {
                let response_status = value.status();
                let response_text = value
                    .text()
                    .await
                    .map_err(|err| error::ApiError::ReqwestError { source: err })?;

                if response_status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    return Err(error::ApiError::TooManyRequests {
                        response: response_text,
                    });
                }

                if !response_status.is_success() {
                    return Err(error::ApiError::parse_from_response_body(&response_text))?;
                }

                todo!();
            }
            Err(err) => Err(error::ApiError::ReqwestError { source: err }),
        }
    }

    pub fn get_price(&self, params: params::GetPrice) {
        // self.get_request_with_params(ApiMethod::GetPrice(params));

        unimplemented!();
    }

    pub fn get_count(&self, params: params::GetCount) {
        // self.get_request_with_params(ApiMethod::GetCount(params));

        unimplemented!();
    }

    pub fn get_country(&self, params: params::GetCountry) {
        // self.get_request_with_params(ApiMethod::GetCountry(params));

        unimplemented!();
    }

    pub fn get_proxy(&self, params: params::GetProxy) {
        // self.get_request_with_params(ApiMethod::GetProxy(params));

        unimplemented!();
    }

    pub fn set_type(&self, params: params::SetType) {
        // self.get_request_with_params(ApiMethod::SetType(params));

        unimplemented!();
    }

    pub fn set_description(&self, params: params::SetDescription) {
        // self.get_request_with_params(ApiMethod::SetDescription(params));

        unimplemented!();
    }

    pub fn buy(&self, params: params::Buy) {
        // self.get_request_with_params(ApiMethod::Buy(params));

        unimplemented!();
    }

    pub fn prolong(&self, params: params::Prolong) {
        // self.get_request_with_params(ApiMethod::Prolong(params));

        unimplemented!();
    }

    pub fn delete(&self, params: params::Delete) {
        // self.get_request_with_params(ApiMethod::Delete(params));

        unimplemented!();
    }

    pub fn check(&self, params: params::Check) {
        // self.get_request_with_params(ApiMethod::Check(params));

        unimplemented!();
    }
}
