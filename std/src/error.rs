use thiserror::Error;
use wasm_bindgen::JsValue;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_wasm_bindgen::Error),

    #[error(transparent)]
    FromHexError(#[from] const_hex::FromHexError),

    #[error("No handler set")]
    NoHandlerSet,

    #[error("No Implement")]
    NoImplement,
}

impl Error {
    pub fn into_error(self) -> js_sys::Error {
        match self {
            Self::SerdeJsonError(e) => {
                let e: JsValue = e.into();
                e.into()
            }
            Self::FromHexError(e) => js_sys::Error::new(&format!("{e}")),
            Self::NoHandlerSet => js_sys::Error::new("No handler set"),
            Self::NoImplement => js_sys::Error::new("No Implement"),
        }
    }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;

/// Result exposed to JS
pub type JsResult<T> = std::result::Result<T, js_sys::Error>;
