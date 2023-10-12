use std::sync::OnceLock;

use async_trait::async_trait;
use wasm_bindgen::JsValue;

use crate::{Error, JsResult, Result};

/// Handler for snap exports
#[async_trait(?Send)]
pub trait Handler: Sync + Send + 'static {
    async fn handle_rpc(&self, _origin: &str, _method: &str, _params: JsValue) -> Result<JsValue> {
        Err(Error::NoImplement)
    }

    async fn handle_transaction(
        &self,
        _transaction: JsValue,
        _chainid: u64,
        _origin: JsValue,
    ) -> Result<JsValue> {
        Err(Error::NoImplement)
    }

    async fn handle_cronjob(&self, _method: &str, _params: JsValue) -> Result<JsValue> {
        Err(Error::NoImplement)
    }
}

impl Handler for () {}

/// Global Handler
static HANDLER: OnceLock<Box<dyn Handler>> = OnceLock::new();

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

/// RPC Request helper function
pub async fn on_rpc_request(origin: &str, method: &str, req: JsValue) -> JsResult<JsValue> {
    let h = HANDLER
        .get()
        .ok_or(Error::NoHandlerSet)
        .map_err(Error::into_error)?;
    h.handle_rpc(origin, method, req)
        .await
        .map_err(Error::into_error)
}

pub async fn on_transaction(transaction: JsValue, chainid: u64, req: JsValue) -> JsResult<JsValue> {
    let h = HANDLER
        .get()
        .ok_or(Error::NoHandlerSet)
        .map_err(Error::into_error)?;
    h.handle_transaction(transaction, chainid, req)
        .await
        .map_err(Error::into_error)
}

pub async fn on_cronjob(method: &str, params: JsValue) -> JsResult<JsValue> {
    let h = HANDLER
        .get()
        .ok_or(Error::NoHandlerSet)
        .map_err(Error::into_error)?;
    h.handle_cronjob(method, params)
        .await
        .map_err(Error::into_error)
}
