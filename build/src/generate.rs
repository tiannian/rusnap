use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use cargo_metadata::MetadataCommand;

use crate::{
    package::{build_package_json, get_cargo_minifest},
    SnapConfig,
};

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

fn build_minifest(path: &Path) -> Result<()> {
    let f = Path::new("./Snap.toml");

    let fc = fs::read_to_string(f)?;

    let config: SnapConfig = toml::from_str(&fc)?;

    let mc = include_str!("../assets/snap.manifest.json");

    let cargo_minifest = get_cargo_minifest()?;

    let content = mc.replace("__RUSNAP_VERSION", &cargo_minifest.package.version);
    let content = content.replace("__RUSNAP_SNAP_DESC", &config.snap.description);
    let content = content.replace("__RUSNAP_SNAP_NAME", &config.snap.name);
    let content = content.replace("__RUSNAP_SNAP_ICON", &config.snap.icon);
    let content = content.replace("__RUSNAP_SNAP_REGISTRY", &config.snap.registry);
    let content = content.replace("__RUSNAP_NAME", &cargo_minifest.package.name);

    // TODO: Add permission parse

    fs::write(path.join("snap.manifest.json"), content)?;

    Ok(())
}

fn build_icon(path: &Path) -> Result<()> {
    let f = fs::read_dir(".")?;

    for p in f {
        let p = p?;

        println!("{:?}", p.file_name());
    }

    Ok(())
}

fn _build() -> Result<()> {
    let path = get_rusnap_path()?;

    fs::create_dir_all(&path)?;

    build_package_json(&path)?;
    build_snap_config(&path)?;
    build_index(&path)?;
    build_icon(&path)?;
    build_minifest(&path)?;

    Ok(())
}

pub fn build() {
    _build().unwrap()
}
