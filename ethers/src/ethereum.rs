use std::fmt::Debug;

use rusnap_utils::{JsResult, RPCRequest};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{Error, Result};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ethereum, js_name = isConnected)]
    fn _is_connected() -> JsValue;

    #[wasm_bindgen(js_namespace = ethereum, js_name = request, catch)]
    async fn _request(req: JsValue) -> JsResult<JsValue>;

    #[wasm_bindgen(js_namespace = ethereum, js_name = on)]
    fn _on(event: &str, func: JsValue);
}

pub fn is_metamask_connected() -> Result<bool> {
    _is_connected().as_bool().ok_or(Error::WrongConnectedType)
}

#[derive(Debug)]
pub struct EthereumProvider {}

impl EthereumProvider {
    async fn _req<T, R>(&self, method: &str, params: T) -> Result<R>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        log::debug!("Send JSONRPC Request: {method} {:?}", params);

        let req = RPCRequest { method, params };

        let req = serde_wasm_bindgen::to_value(&req)?;

        let r = _request(req).await.map_err(Error::JsError)?;

        Ok(serde_wasm_bindgen::from_value(r)?)
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait(?Send)]
impl ethers_providers::JsonRpcClient for EthereumProvider {
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
