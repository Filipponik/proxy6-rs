#![allow(dead_code, unused)]

use reqwest::{Error, Response};

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

    async fn get_request_with_params<TParams: params::ApiParams, TResponse>(
        &self,
        method: &method::ApiMethod,
        params: &TParams,
    ) -> Result<TResponse, error::ApiError> {
        let url = format!(
            "{}/api/{}/{}?{}",
            self.base_url,
            self.api_key,
            method,
            params.to_query_string()
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
                    Err(error::ApiError::TooManyRequests {
                        response: response_text,
                    })
                } else {
                    unimplemented!();
                }
            }
            Err(err) => Err(error::ApiError::ReqwestError { source: err }),
        }
    }

    pub fn get_price(&self) {
        unimplemented!();
    }

    pub fn get_count(&self) {
        unimplemented!();
    }

    pub fn get_country(&self) {
        unimplemented!();
    }

    pub fn get_proxy(&self) {
        unimplemented!();
    }

    pub fn set_type(&self) {
        unimplemented!();
    }

    pub fn set_description(&self) {
        unimplemented!();
    }

    pub fn buy(&self) {
        unimplemented!();
    }

    pub fn prolong(&self) {
        unimplemented!();
    }

    pub fn delete(&self) {
        unimplemented!();
    }

    pub fn check(&self) {
        unimplemented!();
    }
}
