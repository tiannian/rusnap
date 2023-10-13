use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, token::Pub, FnArg, Ident, ItemFn, Visibility};

/// Create a endpoint from an async function
#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let struct_name = input.sig.ident.clone();

    let mod_name = Ident::new(
        &format!("__rusnap_{}", &input.sig.ident),
        Span::call_site().into(),
    );

    input.sig.ident = mod_name.clone();

    let mut arg_name = Vec::with_capacity(input.sig.inputs.len());

    for (index, arg) in input.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(_) = arg {
            let name = Ident::new(&format!("_rh_a{}", index), Span::call_site().into());
            arg_name.push(name);
        }
    }

    let expanded = quote! {

        mod #mod_name {
            use super::*;
            use rusnap::{wasm_bindgen::JsValue, types::{FromRequest, IntoResponse}, Result};

            #input

            #[allow(non_camel_case_types)]
            pub struct #struct_name;

            #[rusnap::async_trait(?Send)]
            impl rusnap::Endpoint for #struct_name {
                async fn handle(
                    &self,
                    method: &str,
                    params: JsValue,
                    data: &dyn std::any::Any,
                    _origin: Option<&str>,
                ) -> Result<JsValue> {

                    #(
                        let #arg_name = FromRequest::from_request(method, params.clone(), data).await;
                    )*

                    let r = #mod_name( #(#arg_name),* ).await;

                    Ok(r.into_response().await?)
                }
            }
        }

        pub use #mod_name::#struct_name;

    };

    TokenStream::from(expanded)
}

/// Main function
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    input.vis = Visibility::Public(Pub::default());

    let expanded = quote! {
        mod __rusnp_entry {
            use rusnap::{wasm_bindgen::{self, JsValue}, wasm_bindgen_futures, exports, JsResult};

            use super::*;

            #[wasm_bindgen::prelude::wasm_bindgen(js_name = _entry)]
            #input

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn on_rpc_request(
                origin: &str,
                method: &str,
                req: JsValue
            ) -> JsResult<JsValue> {
                exports::on_rpc_request(origin, method, req).await
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn on_transaction(
                transaction: JsValue,
                chainid: u64,
                req: JsValue
            ) -> JsResult<JsValue> {
                exports::on_transaction(transaction, chainid, req).await
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn on_cronjob(method: &str, params: JsValue) -> JsResult<JsValue> {
                exports::on_cronjob(method, params).await
            }
        }
    };

    TokenStream::from(expanded)
}
