use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let mod_name = Ident::new(
        &format!("__rusnap_{}", &input.sig.ident),
        Span::call_site().into(),
    );

    let struct_name = input.sig.ident.clone();

    let expanded = quote! {

        #mod_name {
            use super::*;

            #input
        }

        struct #struct_name;

        #[rusnap::async_trait(?Send)]
        impl Endpoint for ExampleEndpoint {
            async fn handle(
                &self,
                method: &str,
                params: JsValue,
                data: &dyn Any,
                _origin: Option<&str>,
            ) -> JsValue {

                let r = #mod_name::#struct_name(method_, params_, data_).await;

                // rusnap::
            }
        }
    };

    TokenStream::from(expanded)
}
