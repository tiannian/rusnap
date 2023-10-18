use std::fs;

use anyhow::Result;
use clap::Args;
use wasm_pack::{
    command::{
        build::{BuildOptions, Target},
        run_wasm_pack, Command,
    },
    install::InstallMode,
};

use crate::utils;

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
    pub fn build_wasm(&self) -> Result<()> {
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

        Ok(())
    }

    pub fn build_js(&self) -> Result<()> {
        Ok(())
    }

    pub fn execute(self) -> Result<()> {
        self.build_wasm()?;

        self.build_js()?;

        Ok(())
    }
}
