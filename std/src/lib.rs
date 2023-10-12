pub mod snap;

mod error;
pub use error::*;

/// Snap exports interface
pub mod exports;
#[doc(inline)]
pub use exports::{set_handler, Handler};

mod route;
pub use route::*;

pub mod types;

/// Re-export `async_trait` from `async-trait` crate
pub use async_trait::async_trait;
pub use serde_wasm_bindgen;
pub use wasm_bindgen;
pub use wasm_bindgen_futures;

pub use rusnap_macros::{handler, main};
