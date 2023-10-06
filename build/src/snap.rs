use anyhow::Result;
use serde_json::Value;

use crate::SnapConfig;

pub(crate) fn build_snap_manifest(s: &str, config: &SnapConfig) -> Result<String> {
    let mut v: Value = serde_json::from_str(s)?;

    if let Some(permission) = &config.permissions {
        permission.build_json(&mut v);
    }

    Ok(serde_json::to_string_pretty(&v)?)
}
