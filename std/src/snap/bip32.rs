use serde::{Deserialize, Serialize};

use crate::{snap::utils, Result};

use super::request;

/// Curve for Bip32
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Curve {
    /// Ed25519 Curve
    Ed25519,
    /// Secp256k1 Curve
    Secp256k1,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bip32Params {
    path: Vec<String>,
    curve: Curve,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed: Option<bool>,
}

/// Entropy for Bip32
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bip32Entropy {
    #[serde(with = "utils")]
    pub chain_code: Vec<u8>,
    pub curve: Curve,
    pub depth: u32,
    pub index: u64,
    pub master_fingerprint: u64,
    pub parent_fingerprint: u64,
    #[serde(with = "utils")]
    pub private_key: Vec<u8>,
    #[serde(with = "utils")]
    pub public_key: Vec<u8>,
}

/// Get entropy in BIP32.
pub async fn get_bip32_entropy(path: &str, curve: Curve) -> Result<Bip32Entropy> {
    let path: Vec<String> = path.split('/').map(String::from).collect();
    let req = Bip32Params {
        path,
        curve,
        compressed: None,
    };
    request("snap_getBip32Entropy", req).await
}

/// Get public key in BIP32.
pub async fn get_bip32_public_key(path: &str, curve: Curve, compressed: bool) -> Result<Vec<u8>> {
    let path: Vec<String> = path.split('/').map(String::from).collect();
    let req = Bip32Params {
        path,
        curve,
        compressed: Some(compressed),
    };
    let r: String = request("snap_getBip32PublicKey", req).await?;

    Ok(const_hex::decode(r)?)
}
