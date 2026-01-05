#[cfg(feature = "async_client")]
pub use async_client::*;
#[cfg(feature = "sync_client")]
pub use sync_client::*;
pub use value_object::*;

mod async_client;
pub(crate) mod deserializer;
pub mod error;
mod method;
pub mod params;
pub mod response;
mod sync_client;
mod value_object;

#[derive(Debug, thiserror::Error)]
pub enum ClientBuildError {
    #[error("API key must be set")]
    ApiKeyMustBeSet,
}

pub type ApiResult<T> = Result<T, error::ApiError>;
