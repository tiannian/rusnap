use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_wasm_bindgen::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
