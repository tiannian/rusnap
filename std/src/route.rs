use std::{any::Any, collections::HashMap};

use async_trait::async_trait;
use wasm_bindgen::JsValue;

use crate::{set_handler, Handler};

/// An endpoint can response an method
#[async_trait(?Send)]
pub trait Endpoint: Send + Sync {
    async fn handle(
        &self,
        methods: &str,
        params: JsValue,
        data: &dyn Any,
        origin: Option<&str>,
    ) -> JsValue;
}

/// Dispatch RPC call based on method.
pub struct Route {
    pub calls: HashMap<String, Box<dyn Endpoint>>,
    pub data: Box<dyn Any + Sync + Send>,
}

#[async_trait(?Send)]
impl Handler for Route {
    // async fn handle_rpc(&self, origin: &str, method: &str, params: JsValue) -> JsValue {
    //     if let Some(h) = self.calls.get(method) {
    //         h.handle(method, params, &self.data, Some(origin)).await
    //     } else {
    //         JsValue::null()
    //     }
    // }
    //
    // async fn handle_cronjob(&self, method: &str, params: JsValue) -> JsValue {
    //     if let Some(h) = self.calls.get(method) {
    //         h.handle(method, params, &self.data, None).await
    //     } else {
    //         JsValue::null()
    //     }
    // }
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

mod tests {
    use crate::types::{self, FromRequest, Method};

    use super::*;

    pub async fn example_handle(
        _method: types::Method,
        _params: types::Params<String>,
        _data: types::Data<&String>,
    ) -> String {
        String::from("Ok")
    }

    pub struct ExampleEndpoint;

    #[async_trait(?Send)]
    impl Endpoint for ExampleEndpoint {
        async fn handle(
            &self,
            method: &str,
            params: JsValue,
            data: &dyn Any,
            _origin: Option<&str>,
        ) -> JsValue {
            let method_ = Method::from_request(method, params.clone(), data).await;
            let params_ = types::Params::from_request(method, params.clone(), data).await;
            let data_ = types::Data::from_request(method, params, data).await;

            let r = example_handle(method_, params_, data_).await;

            serde_wasm_bindgen::to_value(&r).unwrap()
        }
    }
}
