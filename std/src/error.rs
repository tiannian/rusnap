use thiserror::Error;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_wasm_bindgen::Error),
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
