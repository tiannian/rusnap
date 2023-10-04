use std::{fs, process::Command};

use anyhow::{anyhow, Result};
use clap::Args;

use crate::{status::DepInfo, utils};

#[derive(Args, Debug)]
pub struct BuildArg {
    #[clap(short, long, group = "target")]
    pub dev: bool,

    #[clap(short, long, group = "target")]
    pub release: bool,

    #[clap(short, long, group = "target")]
    pub profiling: bool,
}

impl BuildArg {
    pub fn execute(self, info: &DepInfo) -> Result<()> {
        let mut this = self;

        let target_path = utils::get_rusnap_path()?;

        if !target_path.exists() {
            fs::create_dir_all(&target_path)?;
        }

        let mut command = Command::new(info.wasm_pack());

        command
            .arg("build")
            .arg("--out-dir")
            .arg(target_path.join("pkg"))
            .arg("--target")
            .arg("web")
            .arg("--no-typescript")
            .arg("--out-name")
            .arg("__rusnap")
            .arg("--no-pack");

        if !this.dev && !this.release && !this.profiling {
            this.dev = true;
        }

        if this.dev {
            command.arg("--dev");
        }

        if this.release {
            command.arg("--release");
        }

        if this.profiling {
            command.arg("--profiling");
        }

        let res = command.spawn()?.wait()?;

        if !res.success() {
            return Err(anyhow!("Build webassembly failed"));
        }

        // Install info
        let nm_path = target_path.join("node_modules");
        if !nm_path.exists() {
            let mut cmd = info.npm_install_deps().ok_or(anyhow!("No npm found"))?;
            let res = cmd.current_dir(&target_path).spawn()?.wait()?;
            if !res.success() {
                return Err(anyhow!("Install nodejs dependenices failed"));
            }
        }

        // Build mm-snap
        let mut cmd = info.npm_run().ok_or(anyhow!("No npm found"))?;

        let res = cmd.arg("build").current_dir(&target_path).spawn()?.wait()?;

        if res.success() {
            Ok(())
        } else {
            Err(anyhow!("Failed to build snap"))
        }
    }
}
