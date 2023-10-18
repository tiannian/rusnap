use rusnap_utils::{JsResult, RPCRequest};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{Error, Result};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = snap, js_name = request, catch)]
    async fn _request(args: JsValue) -> JsResult<JsValue>;
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

    let sers = Serializer::default().serialize_maps_as_objects(true);

    let req = req.serialize(&sers)?;

    log::debug!("Rpc Call Request is: {:?}", req);

    let resp = _request(req).await.map_err(Error::JsError)?;

    log::debug!("Rpc Call Result is: {:?}", resp);

    Ok(serde_wasm_bindgen::from_value(resp)?)
}
