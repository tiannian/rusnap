use rusnap::types;
use rusnap_macros::handler;

#[handler]
pub async fn example_handle(
    _method: types::Method,
    _params: types::Params<String>,
    _data: types::Data<&String>,
) -> String {
    String::from("Ok")
}

fn main() {}
