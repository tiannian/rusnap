use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    pub origin: String,
    pub request: RpcRequest,
}

#[async_trait(?Send)]
pub trait Handler {
    async fn handle_rpc(&mut self, _req: JsValue) -> JsValue {
        JsValue::null()
    }

    async fn handle_transaction(&mut self, _req: JsValue) -> JsValue {
        JsValue::null()
    }

    async fn handle_cronjob(&mut self, _req: JsValue) -> JsValue {
        JsValue::null()
    }
}

impl Handler for () {}

#[macro_export]
macro_rules! handler {
    ($g:block) => {
        mod __handler {

            use $crate::exports::Handler;
            use $crate::wasm_bindgen::{prelude::wasm_bindgen, JsValue};
            use $crate::wasm_bindgen_futures;

            #[wasm_bindgen(js_name = onRpcRequest)]
            pub async fn _on_rpc(req: JsValue) -> JsValue {
                let mut handler = $g;

                handler.handle_rpc(req).await
            }

            #[wasm_bindgen(js_name = onTransaction)]
            pub async fn _on_transaction(req: JsValue) -> JsValue {
                let mut handler = $g;

                handler.handle_transaction(req).await
            }

            #[wasm_bindgen(js_name = onCronjob)]
            pub async fn _on_cronjob(req: JsValue) -> JsValue {
                let mut handler = $g;

                handler.handle_cronjob(req).await
            }
        }
    };
}

#[macro_export]
macro_rules! entry {
    ($g:block) => {
        mod __entry {
            use $crate::wasm_bindgen;
            use $crate::wasm_bindgen_futures;

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn _entry() {
                let entry = async $g;

                entry.await;
            }
        }
    };
}
