use std::fs;

use anyhow::{anyhow, Result};
use clap::Args;
use wasm_pack::{
    command::{
        build::{BuildOptions, Target},
        run_wasm_pack, Command,
    },
    install::InstallMode,
};

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
        let target_path = utils::get_rusnap_path()?;

        if !target_path.exists() {
            fs::create_dir_all(&target_path)?;
        }

        let build_command = BuildOptions {
            path: None,
            scope: None,
            mode: InstallMode::Normal,
            disable_dts: true,
            weak_refs: false,
            reference_types: false,
            target: Target::Web,
            debug: false,
            dev: self.dev,
            release: self.release,
            profiling: self.profiling,
            out_dir: target_path
                .join("pkg")
                .to_str()
                .expect("Failed into str")
                .into(),
            out_name: Some("__rusnap".into()),
            no_pack: true,
            extra_options: vec![],
        };

        run_wasm_pack(Command::Build(build_command))?;

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
