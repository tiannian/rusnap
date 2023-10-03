use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use cargo_metadata::MetadataCommand;

use crate::package::build_package_json;

fn get_rusnap_path() -> Result<PathBuf> {
    let metadata = MetadataCommand::new().exec()?;

    Ok(metadata.workspace_root.join("target").join("rusnap").into())
}

fn build_snap_config(path: &Path) -> Result<()> {
    let c = include_str!("../assets/snap.config.js");
    let f = path.join("snap.config.js");

    if f.exists() {
        return Ok(());
    }

    fs::write(f, c)?;

    Ok(())
}

fn build_index(path: &Path) -> Result<()> {
    let c = include_str!("../assets/index.js");
    let f = path.join("index.js");

    if f.exists() {
        return Ok(());
    }

    fs::write(f, c)?;

    Ok(())
}

fn _build() -> Result<()> {
    let path = get_rusnap_path()?;

    fs::create_dir_all(&path)?;

    build_package_json(&path)?;
    build_snap_config(&path)?;
    build_index(&path)?;

    Ok(())
}

pub fn build() {
    _build().unwrap()
}
