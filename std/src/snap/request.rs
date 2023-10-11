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

/// Call metamask using `snap.request`.
///
/// This function used to call restricted methods.
///
/// Snap Document: [Restricted methods](https://docs.metamask.io/snaps/reference/rpc-api/#restricted-methods)
pub async fn request<P, R>(method: &'static str, params: P) -> Result<R>
where
    P: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let req = Request { method, params };
    let req = serde_wasm_bindgen::to_value(&req)?;

    log::debug!("Rpc Call Request is: {:?}", req);

    let resp = _request(req).await;

    log::debug!("Rpc Call Result is: {:?}", resp);

    Ok(serde_wasm_bindgen::from_value(resp)?)
}
