use rusnap_utils::bytes;
use serde::{Deserialize, Serialize};

use crate::Result;

use super::request;

/// Entropy for Bip32
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bip44Entropy {
    #[serde(with = "bytes")]
    pub chain_code: Vec<u8>,

    #[serde(rename = "coin_type")]
    pub coin_type: u64,

    pub depth: u32,

    pub index: u64,

    pub master_fingerprint: u64,

    pub parent_fingerprint: u64,

    pub path: String,

    #[serde(with = "bytes")]
    pub private_key: Vec<u8>,

    #[serde(with = "bytes")]
    pub public_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bip44Params {
    #[serde(rename = "coinType")]
    pub coin_type: u64,
}

/// Get entropy in EIP44.
pub async fn get_bip44_entropy(coin_type: u64) -> Result<Bip44Entropy> {
    let params = Bip44Params { coin_type };

    Ok(request("snap_getBip44Entropy", params).await?)
}
