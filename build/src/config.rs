use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    pub path: String,
    pub curve: Curve,
}

impl Bip32 {
    pub fn to_json(&self) -> Value {
        let s: Vec<String> = self.path.split('/').map(String::from).collect();

        json!({ "path": s, "curve": self.curve })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bip44 {
    pub coin_type: u64,
}

impl Bip44 {
    pub fn to_json(&self) -> Value {
        json!({
            "coinType": self.coin_type
        })
    }
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

impl Cronjob {
    pub fn to_json(&self) -> Value {
        json!({
            "expression": self.expression,
            "request": {
                "method": self.request.method,
                "params": self.request.params
            }
        })
    }
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
pub struct Empty {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapPermissions {
    pub dialog: Option<Empty>,
    pub bip32_entropy: Option<Vec<Bip32>>,
    pub bip32_public_key: Option<Vec<Bip32>>,
    pub bip44_entropy: Option<Vec<Bip44>>,
    pub entropy: Option<Empty>,
    pub manage_accounts: Option<Empty>,
    pub manage_state: Option<Empty>,
    pub notify: Option<Empty>,
    pub cronjob: Option<Vec<Cronjob>>,
    pub ethereum_provider: Option<Empty>,
    pub network_access: Option<Empty>,
    pub rpc: Option<RPC>,
    pub transaction_insight: Option<TransactionInsight>,
}

impl SnapPermissions {
    pub fn build_json(&self, v: &mut Value) {
        if self.dialog.is_some() {
            v["snap_dialog"] = json!({});
        }

        if let Some(vs) = &self.bip32_entropy {
            let vs: Vec<Value> = vs.iter().map(Bip32::to_json).collect();
            v["snap_getBip32Entropy"] = Value::Array(vs);
        }

        if let Some(vs) = &self.bip32_public_key {
            let vs: Vec<Value> = vs.iter().map(Bip32::to_json).collect();
            v["snap_getBip32PublicKey"] = Value::Array(vs);
        }

        if let Some(vs) = &self.bip44_entropy {
            let vs: Vec<Value> = vs.iter().map(Bip44::to_json).collect();
            v["snap_getBip44Entropy"] = Value::Array(vs);
        }

        if self.entropy.is_some() {
            v["snap_getEntropy"] = json!({});
        }

        if self.manage_accounts.is_some() {
            v["snap_manageAccounts"] = json!({});
        }

        if self.manage_state.is_some() {
            v["snap_manageState"] = json!({});
        }

        if self.notify.is_some() {
            v["snap_notify"] = json!({});
        }

        if let Some(vs) = &self.cronjob {
            let vs: Vec<Value> = vs.iter().map(Cronjob::to_json).collect();
            v["endowment:cronjob"] = Value::Array(vs);
        }

        if self.ethereum_provider.is_some() {
            v["endowment:ethereum-provider"] = json!({});
        }

        if self.network_access.is_some() {
            v["endowment:network-access"] = json!({});
        }

        if let Some(rpc) = &self.rpc {
            v["endowment:rpc"] = json!({
                "dapps": rpc.dapps,
                "snaps": rpc.snaps
            });
        }

        if let Some(e) = &self.transaction_insight {
            v["endowment:transaction-insight"] = json!({
                "allowTransactionOrigin": e.allow_transaction_origin
            });
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapConfig {
    pub snap: SnapPackage,
    pub permissions: Option<SnapPermissions>,
}

#[cfg(test)]
mod tests {
    use crate::SnapConfig;

    #[test]
    fn test_config() {
        let s = include_str!("../assets/Snap.toml");

        let o: SnapConfig = toml::from_str(s).unwrap();

        println!("{:#?}", o);
    }
}
