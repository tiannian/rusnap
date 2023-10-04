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

pub fn parse_rpc_request(req: JsValue) -> (String, RpcRequest) {
    let req: Request = serde_wasm_bindgen::from_value(req).unwrap();

    (req.origin, req.request)
}

#[macro_export]
macro_rules! rpc_handler {
    ($handler:ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = onRpcRequest)]
        pub async fn _on_rpc_request(req: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue {
            let (origin, request) = $crate::exports::parse_rpc_request(req);

            let res = $handler(origin, request);

            $crate::serde_wasm_bindgen::to_value(&res).unwrap()
        }
    };
}
