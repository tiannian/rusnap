use std::sync::OnceLock;

use async_trait::async_trait;
use wasm_bindgen::JsValue;

/// Handler for snap exports
#[async_trait(?Send)]
pub trait Handler: Sync + Send + 'static {
    async fn handle_rpc(&self, _origin: &str, _method: &str, _params: JsValue) -> JsValue {
        JsValue::null()
    }

    async fn handle_transaction(
        &self,
        _transaction: JsValue,
        _chainid: u64,
        _origin: JsValue,
    ) -> JsValue {
        JsValue::null()
    }

    async fn handle_cronjob(&self, _method: &str, _params: JsValue) -> JsValue {
        JsValue::null()
    }
}

impl Handler for () {}

/// Global Handler
pub static HANDLER: OnceLock<Box<dyn Handler>> = OnceLock::new();

/// Set Global handler for snap exports
pub fn set_handler(handler: impl Handler) {
    let handler = Box::new(handler);

    if HANDLER.get().is_none() {
        HANDLER
            .set(handler)
            .map_err(|_| String::from("Failed to set handler"))
            .unwrap();
    }
}

/// Mark an async function is main.
///
/// This function will call when snap load.
#[macro_export]
macro_rules! entry {
    ($g:ident) => {
        mod __entry {
            use super::*;
            use $crate::exports::HANDLER;
            use $crate::wasm_bindgen::{self, JsValue};
            use $crate::wasm_bindgen_futures;

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn _entry() {
                $g().await;
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn on_rpc_request(origin: &str, method: &str, req: JsValue) -> JsValue {
                if let Some(h) = HANDLER.get() {
                    h.handle_rpc(origin, method, req).await
                } else {
                    JsValue::null()
                }
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn on_transaction(
                transaction: JsValue,
                chainid: u64,
                req: JsValue,
            ) -> JsValue {
                if let Some(h) = HANDLER.get() {
                    h.handle_transaction(transaction, chainid, req).await
                } else {
                    JsValue::null()
                }
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn on_cronjob(method: &str, params: JsValue) -> JsValue {
                if let Some(h) = HANDLER.get() {
                    h.handle_cronjob(method, params).await
                } else {
                    JsValue::null()
                }
            }
        }
    };
}
