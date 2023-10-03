use std::path::PathBuf;

use anyhow::Result;
use cargo_metadata::MetadataCommand;

use crate::package::build_package_json;

fn get_rusnap_path() -> Result<PathBuf> {
    let metadata = MetadataCommand::new().exec()?;

    Ok(metadata.workspace_root.join("target").join("rusnap").into())
}

fn _build() -> Result<()> {
    let path = get_rusnap_path()?;

    build_package_json(&path)?;

    Ok(())
}

pub fn build() {
    _build().unwrap()
}
