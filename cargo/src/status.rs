use anyhow::Result;

pub enum Npm {
    Npm(String),
    Yarn(String),
    Pnpm(String),
}

pub struct DepInfo {
    pub npm: Npm,
    pub wasm_pack: String,
}

pub fn check_deps() -> Result<DepInfo> {
    let info = DepInfo {
        npm: Npm::Npm(String::new()),
        wasm_pack: String::new(),
    };

    Ok(info)
}
