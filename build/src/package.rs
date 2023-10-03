use std::{env, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CargoPackage {
    name: String,
    version: String,
    authors: Vec<String>,
    #[serde(default)]
    license: String,
    #[serde(default)]
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoFile {
    pub package: CargoPackage,
}

pub(crate) fn build_package_json(path: &Path) -> Result<()> {
    let path = path.join("package.json");

    if path.exists() {
        return Ok(());
    }

    let content = include_str!("../assets/package.json");

    let mp = env::var("CARGO_MANIFEST_DIR")?;

    let mpath = Path::new(&mp).join("Cargo.toml");

    let ms = fs::read_to_string(mpath)?;

    let pkg: CargoFile = toml::from_str(&ms)?;

    let content = content.replace("__RUSNAP_NAME", &pkg.package.name);
    let content = content.replace("__RUSNAP_VERSION", &pkg.package.version);
    let content = content.replace("__RUSNAP_AUTHOR", &pkg.package.authors[0]);

    fs::write(path, content)?;

    Ok(())
}
