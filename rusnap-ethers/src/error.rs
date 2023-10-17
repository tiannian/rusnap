use ethers_providers::ProviderError;
use thiserror::Error;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SerdeError(#[from] serde_wasm_bindgen::Error),

    #[error("{0:?}")]
    JsError(js_sys::Error),
}

impl Error {
    pub fn into_provider_error(self) -> ProviderError {
        ProviderError::CustomError(format!("{}", self))
    }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
