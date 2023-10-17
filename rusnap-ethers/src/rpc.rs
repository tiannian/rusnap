use std::fmt::Debug;

use js_sys::{Object, Reflect};
use rusnap_utils::JsResult;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{Error, Result};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ethereum, js_name = isConnected)]
    fn _is_connected() -> bool;

    #[wasm_bindgen(js_namespace = ethereum, js_name = request, catch)]
    async fn _request(req: JsValue) -> JsResult<JsValue>;

    #[wasm_bindgen(js_namespace = ethereum, js_name = on)]
    fn _on(event: &str, func: JsValue);
}

/// Check metamask is connected
pub fn is_metamask_connected() -> bool {
    _is_connected()
}

/// RPC to request metamask
#[derive(Debug, Default)]
pub struct MetamaskRpc {}

impl MetamaskRpc {
    async fn _req<T, R>(&self, method: &str, params: T) -> Result<R>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let req = Object::new();
        Reflect::set(&req, &"method".into(), &method.into())
            .map_err(|e| Error::JsError(e.into()))?;

        let params = serde_wasm_bindgen::to_value(&params)?;
        if !params.is_undefined() {
            Reflect::set(&req, &"params".into(), &params).map_err(|e| Error::JsError(e.into()))?;
        }

        log::debug!("Send JSONRPC Request: {req:?}");

        let r = _request(req.into()).await;

        if let Err(e) = &r {
            log::error!("Got error: {e:?}");
        }

        let r = r.map_err(Error::JsError)?;

        Ok(serde_wasm_bindgen::from_value(r)?)
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait(?Send)]
impl ethers_providers::JsonRpcClient for MetamaskRpc {
    type Error = ethers_providers::ProviderError;

    async fn request<T, R>(&self, method: &str, params: T) -> std::result::Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        self._req(method, params)
            .await
            .map_err(Error::into_provider_error)
    }
}
