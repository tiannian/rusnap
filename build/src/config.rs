use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapPackage {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub registry: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Curve {
    Secp256k1,
    Ed25519,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bip32 {
    pub path: Vec<String>,
    pub curve: Curve,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bip44 {
    pub coin_type: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CronjobRequest {
    pub method: String,
    pub params: toml::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cronjob {
    pub expression: String,
    pub request: CronjobRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cron {
    pub jobs: Vec<Cronjob>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RPC {
    pub dapps: bool,
    pub snaps: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInsight {
    pub allow_transaction_origin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "permission")]
pub enum SnapPermission {
    Dialog,
    GetBip32Entropy(Vec<Bip32>),
    GetBip32PublicKey(Vec<Bip32>),
    GetBip44Entropy(Vec<Bip44>),
    GetEntropy,
    ManageAccounts,
    ManageState,
    Notify,
    Cronjob(Cron),
    EthereumProvider,
    NetworkAccess,
    RPC(RPC),
    TransactionInsight(TransactionInsight),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapConfig {
    pub snap: SnapPackage,
    pub permissions: Vec<SnapPermission>,
}
