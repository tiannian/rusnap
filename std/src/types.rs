//! Helper function and type

use std::any::Any;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::Error;

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
    P: for<'de> Deserialize<'de>,
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

#[async_trait(?Send)]
pub trait IntoResponse: Sized {
    async fn into_response(self) -> Result<JsValue, Error>;
}

#[async_trait(?Send)]
impl IntoResponse for () {
    async fn into_response(self) -> Result<JsValue, Error> {
        Ok(JsValue::null())
    }
}

macro_rules! define_concrate_type_response {
    ($t:ty) => {
        #[async_trait(?Send)]
        impl IntoResponse for $t {
            async fn into_response(self) -> Result<JsValue, Error> {
                Ok(JsValue::from(self))
            }
        }
    };
}

define_concrate_type_response!(bool);
define_concrate_type_response!(f32);
define_concrate_type_response!(f64);
define_concrate_type_response!(i128);
define_concrate_type_response!(i64);
define_concrate_type_response!(i32);
define_concrate_type_response!(i16);
define_concrate_type_response!(i8);
define_concrate_type_response!(u128);
define_concrate_type_response!(u64);
define_concrate_type_response!(u32);
define_concrate_type_response!(u16);
define_concrate_type_response!(u8);
define_concrate_type_response!(isize);
define_concrate_type_response!(usize);

#[async_trait(?Send)]
impl<'a> IntoResponse for &'a String {
    async fn into_response(self) -> Result<JsValue, Error> {
        Ok(JsValue::from(self))
    }
}

#[async_trait(?Send)]
impl<T> IntoResponse for Option<T>
where
    JsValue: From<T>,
{
    async fn into_response(self) -> Result<JsValue, Error> {
        Ok(match self {
            Some(v) => JsValue::from(v),
            None => JsValue::null(),
        })
    }
}

#[async_trait(?Send)]
impl<T, E> IntoResponse for Result<T, E>
where
    T: Serialize,
    Error: From<E>,
{
    async fn into_response(self) -> Result<JsValue, Error> {
        let r = serde_wasm_bindgen::to_value(&self?)?;

        Ok(r)
    }
}
