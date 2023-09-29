use std::process::Command;

use anyhow::Result;
use clap::Args;

use crate::status::DepInfo;

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
        let mut command = Command::new(info.wasm_pack());

        command.arg("build");

        if self.dev {
            command.arg("--dev");
        }

        if self.release {
            command.arg("--release");
        }

        if self.profiling {
            command.arg("--profiling");
        }

        command.spawn()?.wait()?;

        Ok(())
    }
}
