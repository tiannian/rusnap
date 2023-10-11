use serde::{Deserialize, Serialize};

use crate::Result;

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

pub async fn get_bip32_entropy(path: &str, curve: Curve) -> Result<Bip32Entropy> {
    let path: Vec<String> = path.split('/').map(String::from).collect();
    let req = Bip32Params {
        path,
        curve,
        compressed: None,
    };
    request("snap_getBip32Entropy", req).await
}

pub async fn get_bip32_public_key(path: &str, curve: Curve, compressed: bool) -> Result<Vec<u8>> {
    let path: Vec<String> = path.split('/').map(String::from).collect();
    let req = Bip32Params {
        path,
        curve,
        compressed: Some(compressed),
    };
    let r: String = request("snap_getBip32Entropy", req).await?;

    Ok(const_hex::decode(r)?)
}

/// Utils
pub mod utils {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize_bytes<S, T>(x: T, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        s.serialize_str(&const_hex::encode_prefixed(x))
    }

    pub fn deserialize_bytes<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(d)?;
        const_hex::decode(value)
            .map(Into::into)
            .map_err(serde::de::Error::custom)
    }

    pub use deserialize_bytes as deserialize;
    pub use serialize_bytes as serialize;
}
