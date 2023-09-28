use primitive_types::{H256, H512};
use serde::{Deserialize, Serialize};

use crate::Result;

use super::request;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Curve {
    Ed25519,
    Secp256k1,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bip32Params {
    path: Vec<String>,
    curve: Curve,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bip32Entropy {
    pub chain_code: H256,
    pub curve: Curve,
    pub depth: u32,
    pub index: u64,
    pub master_fingerprint: u64,
    pub parent_fingerprint: u64,
    pub private_key: H256,
    pub public_key: H512,
}

pub async fn get_bip32_entropy(path: &str, curve: Curve) -> Result<Bip32Entropy> {
    let path: Vec<String> = path.split('/').map(String::from).collect();

    let req = Bip32Params { path, curve };

    request("snap_getBip32Entropy", req).await
}
