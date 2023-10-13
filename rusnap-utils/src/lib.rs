/// Result exposed to JS
pub type JsResult<T> = std::result::Result<T, js_sys::Error>;

pub mod bytes;

mod request;
pub use request::*;
