use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::Result;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = snap, js_name = request)]
    async fn _request(args: JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize)]
struct Request<P> {
    method: &'static str,
    params: P,
}

pub async fn request<P, R>(method: &'static str, params: P) -> Result<R>
where
    P: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let req = Request { method, params };
    let req = serde_wasm_bindgen::to_value(&req)?;

    let resp = _request(req).await;
    Ok(serde_wasm_bindgen::from_value(resp)?)
}
