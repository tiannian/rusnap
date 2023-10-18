use std::process::Command;

use anyhow::Result;

#[derive(Debug)]
pub enum NpmInfo {
    Npm(String),
    Yarn(String),
    Pnpm(String),
    None,
}

#[derive(Debug)]
pub struct DepInfo {
    pub npm: NpmInfo,
    pub wasm_pack: Option<String>,
}

impl DepInfo {
    pub fn new() -> Result<Self> {
        let wasm_pack = Command::new("wasm-pack").arg("-V").output()?;
        let wasm_pack = if wasm_pack.status.success() {
            let v = String::from_utf8(wasm_pack.stdout)?;

            let mut s = v.trim().split(' ');
            s.next();
            s.next().map(String::from)
        } else {
            None
        };

        {
            let output = Command::new("npm").arg("-v").output()?;

            let v = String::from_utf8(output.stdout)?;

            if output.status.success() {
                return Ok(Self {
                    wasm_pack,
                    npm: NpmInfo::Npm(String::from(v.trim())),
                });
            }
        }

        {
            let output = Command::new("yarn").arg("-v").output()?;

            if output.status.success() {
                return Ok(Self {
                    wasm_pack,
                    npm: NpmInfo::Yarn(String::from_utf8(output.stdout)?),
                });
            }
        }

        {
            let output = Command::new("pnpm").arg("-v").output()?;

            if output.status.success() {
                return Ok(Self {
                    wasm_pack,
                    npm: NpmInfo::Pnpm(String::from_utf8(output.stdout)?),
                });
            }
        }

        Ok(Self {
            wasm_pack,
            npm: NpmInfo::None,
        })
    }

    pub fn is_right(&self) -> bool {
        !matches!(self.npm, NpmInfo::None) && self.wasm_pack.is_some()
    }

    pub fn npm_install_deps(&self) -> Option<Command> {
        match self.npm {
            NpmInfo::None => None,
            NpmInfo::Npm(_) => {
                let mut cmd = Command::new("npm");

                cmd.arg("install");

                Some(cmd)
            }
            NpmInfo::Yarn(_) => {
                let mut cmd = Command::new("yarn");

                cmd.arg("install");

                Some(cmd)
            }
            NpmInfo::Pnpm(_) => {
                let mut cmd = Command::new("pnpm");

                cmd.arg("install");

                Some(cmd)
            }
        }
    }

    pub fn npm_run(&self) -> Option<Command> {
        match self.npm {
            NpmInfo::None => None,
            NpmInfo::Npm(_) => {
                let mut cmd = Command::new("npm");

                cmd.arg("run");

                Some(cmd)
            }
            NpmInfo::Yarn(_) => {
                let mut cmd = Command::new("yarn");

                cmd.arg("run");

                Some(cmd)
            }
            NpmInfo::Pnpm(_) => {
                let mut cmd = Command::new("pnpm");

                cmd.arg("run");

                Some(cmd)
            }
        }
    }
}
