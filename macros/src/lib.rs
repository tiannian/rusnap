use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn};

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
            use rusnap::{wasm_bindgen::JsValue, types::FromRequest};

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
                ) -> JsValue {

                    #(
                        let #arg_name = FromRequest::from_request(method, params.clone(), data).await;
                    )*

                    let r = #mod_name( #(#arg_name),* ).await;

                    // rusnap::
                    rusnap::serde_wasm_bindgen::to_value(&r).unwrap()
                }
            }
        }

        pub use #mod_name::#struct_name;

    };

    TokenStream::from(expanded)
}
