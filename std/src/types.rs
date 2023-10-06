use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    pub origin: String,
    pub request: RpcRequest,
}
