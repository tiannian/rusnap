use std::path::PathBuf;

use anyhow::Result;
use cargo_metadata::MetadataCommand;

pub fn get_rusnap_path() -> Result<PathBuf> {
    let metadata = MetadataCommand::new().exec()?;

    Ok(metadata.workspace_root.join("target").join("rusnap").into())
}
