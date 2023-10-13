use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCRequest<'a, P> {
    pub method: &'a str,
    pub params: P,
}
