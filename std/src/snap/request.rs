use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{Error, JsResult, Result};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = snap, js_name = request, catch)]
    async fn _request(args: JsValue) -> JsResult<JsValue>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCRequest<'a, P> {
    pub method: &'a str,
    pub params: P,
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
    let req = RPCRequest { method, params };
    let req = serde_wasm_bindgen::to_value(&req)?;

    log::debug!("Rpc Call Request is: {:?}", req);

    let resp = _request(req).await.map_err(Error::JsError)?;

    log::debug!("Rpc Call Result is: {:?}", resp);

    Ok(serde_wasm_bindgen::from_value(resp)?)
}
