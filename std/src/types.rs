//! Helper function and type

use std::any::Any;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

/// Helper trait from request
#[async_trait(?Send)]
pub trait FromRequest<'a> {
    async fn from_request(method: &str, params: JsValue, data: &'a dyn Any) -> Self;
}

/// JSONRPC Method
pub struct Method(pub String);

#[async_trait(?Send)]
impl<'a> FromRequest<'a> for Method {
    async fn from_request(method: &str, _params: JsValue, _data: &'a dyn Any) -> Self {
        Self(String::from(method))
    }
}

/// JSONRPC Params
pub struct Params<P>(pub P);

#[async_trait(?Send)]
impl<'a, P> FromRequest<'a> for Params<P>
where
    P: DeserializeOwned,
{
    async fn from_request(_method: &str, params: JsValue, _data: &'a dyn Any) -> Self {
        let params = serde_wasm_bindgen::from_value(params).unwrap();

        Self(params)
    }
}

/// JSONRPC Data
pub struct Data<D>(pub D);

#[async_trait(?Send)]
impl<'a, D> FromRequest<'a> for Data<&'a D>
where
    D: 'static,
{
    async fn from_request(_method: &str, _params: JsValue, data: &'a dyn Any) -> Self {
        let data = data.downcast_ref().unwrap();

        Self(data)
    }
}
