use std::{any::Any, collections::HashMap};

use async_trait::async_trait;
use wasm_bindgen::JsValue;

use crate::{set_handler, Error, Handler, JsResult};

/// An endpoint can response an method
#[async_trait(?Send)]
pub trait Endpoint: Send + Sync {
    async fn handle(
        &self,
        methods: &str,
        params: JsValue,
        data: &dyn Any,
        origin: Option<&str>,
    ) -> JsResult<JsValue>;
}

/// Dispatch RPC call based on method.
pub struct Route {
    pub calls: HashMap<String, Box<dyn Endpoint>>,
    pub data: Box<dyn Any + Sync + Send>,
}

#[async_trait(?Send)]
impl Handler for Route {
    async fn handle_rpc(&self, origin: &str, method: &str, params: JsValue) -> JsResult<JsValue> {
        let h = self
            .calls
            .get(method)
            .ok_or(Error::NoTargetMethodFound.into_error())?;

        h.handle(method, params, &self.data, Some(origin)).await
    }

    async fn handle_cronjob(&self, method: &str, params: JsValue) -> JsResult<JsValue> {
        let h = self
            .calls
            .get(method)
            .ok_or(Error::NoTargetMethodFound.into_error())?;
        h.handle(method, params, &self.data, None).await
    }
}

impl Route {
    /// Create a Route
    ///
    /// `data` is an share object for each RPC call.
    pub fn new<D>(data: D) -> Self
    where
        D: Send + Sync + 'static,
    {
        let data: Box<dyn Any + Send + Sync> = Box::new(data);

        Self {
            calls: HashMap::new(),
            data,
        }
    }

    /// Add an Endpoint to the specified method.
    pub fn at(mut self, method: &str, endpoint: impl Endpoint + 'static) -> Self {
        self.calls.insert(String::from(method), Box::new(endpoint));
        self
    }

    /// Serve Route in entry function.
    pub fn serve(self) {
        set_handler(self)
    }
}
