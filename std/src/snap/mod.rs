//! Snap JSONRPC API
//!
//! Snap Document: [Snaps JSON-RPC API](https://docs.metamask.io/snaps/reference/rpc-api/)

mod dialog;
pub use dialog::*;

mod bip32;
pub use bip32::*;

mod bip44;
pub use bip44::*;

mod request;
pub use request::*;

mod notify;
pub use notify::*;

mod state;
pub use state::*;

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
