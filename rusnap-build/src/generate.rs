use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use cargo_metadata::MetadataCommand;

use crate::{
    package::{build_package_json, get_cargo_minifest},
    snap::build_snap_manifest,
    SnapConfig,
};

fn get_config() -> Result<SnapConfig> {
    let f = Path::new("./Snap.toml");

    let fc = fs::read_to_string(f)?;

    let config: SnapConfig = toml::from_str(&fc)?;

    Ok(config)
}

fn get_rusnap_path() -> Result<PathBuf> {
    let metadata = MetadataCommand::new().exec()?;

    Ok(metadata.workspace_root.join("target").join("rusnap").into())
}

fn build_minifest(path: &Path, config: &SnapConfig) -> Result<()> {
    let mc = include_str!("../assets/snap.manifest.json");

    let cargo_minifest = get_cargo_minifest()?;

    let content = mc.replace("__RUSNAP_VERSION", &cargo_minifest.package.version);
    let content = content.replace("__RUSNAP_SNAP_DESC", &config.snap.description);
    let content = content.replace("__RUSNAP_SNAP_NAME", &config.snap.name);
    let content = content.replace("__RUSNAP_SNAP_ICON", &config.snap.icon);
    let content = content.replace("__RUSNAP_SNAP_REGISTRY", &config.snap.registry);
    let content = content.replace("__RUSNAP_NAME", &cargo_minifest.package.name);

    let content = build_snap_manifest(&content, config)?;

    fs::write(path.join("snap.manifest.json"), content)?;

    Ok(())
}

fn build_icon(path: &Path, config: &SnapConfig) -> Result<()> {
    let target = path.join("icon");

    fs::copy(&config.snap.icon, target)?;

    Ok(())
}

fn _build() -> Result<()> {
    let path = get_rusnap_path()?;

    fs::create_dir_all(&path)?;

    let config = get_config()?;

    build_package_json(&path)?;
    build_icon(&path, &config)?;
    build_minifest(&path, &config)?;

    Ok(())
}

pub fn build() {
    _build().unwrap()
}
