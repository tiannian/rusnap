use serde::{Deserialize, Serialize};

use crate::Result;

use super::request;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntropyParams {
    pub version: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

pub async fn get_entropy_v1_salt<S>(salt: S) -> Result<Vec<u8>>
where
    S: Into<String>,
{
    let params = EntropyParams {
        version: 1,
        salt: Some(salt.into()),
    };

    let r: String = request("snap_getEntropy", params).await?;

    Ok(const_hex::decode(r)?)
}

pub async fn get_entropy_v1() -> Result<Vec<u8>> {
    let params = EntropyParams {
        version: 1,
        salt: None,
    };

    let r: String = request("snap_getEntropy", params).await?;

    Ok(const_hex::decode(r)?)
}
