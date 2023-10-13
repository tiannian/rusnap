use thiserror::Error;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error("Error from js")]
    JsError(js_sys::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_wasm_bindgen::Error),

    #[error(transparent)]
    FromHexError(#[from] const_hex::FromHexError),
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
