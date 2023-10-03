use std::{env, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CargoPackage {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CargoFile {
    pub package: CargoPackage,
}

pub(crate) fn get_cargo_minifest() -> Result<CargoFile> {
    let mp = env::var("CARGO_MANIFEST_DIR")?;

    let mpath = Path::new(&mp).join("Cargo.toml");

    let ms = fs::read_to_string(mpath)?;

    let pkg: CargoFile = toml::from_str(&ms)?;

    Ok(pkg)
}

pub(crate) fn build_package_json(path: &Path) -> Result<()> {
    let path = path.join("package.json");

    if path.exists() {
        return Ok(());
    }

    let content = include_str!("../assets/package.json");

    let pkg = get_cargo_minifest()?;

    let content = content.replace("__RUSNAP_NAME", &pkg.package.name);
    let content = content.replace("__RUSNAP_VERSION", &pkg.package.version);
    let content = content.replace("__RUSNAP_AUTHOR", &pkg.package.authors[0]);

    fs::write(path, content)?;

    Ok(())
}
