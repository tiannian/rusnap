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

    #[error("No target method found")]
    NoTargetMethodFound,
}

impl Error {
    pub fn into_error(self) -> js_sys::Error {
        match self {
            Self::SerdeJsonError(e) => {
                let e: JsValue = e.into();
                e.into()
            }

            _ => js_sys::Error::new(&format!("{self}")),
        }
    }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;

/// Result exposed to JS
pub type JsResult<T> = std::result::Result<T, js_sys::Error>;
