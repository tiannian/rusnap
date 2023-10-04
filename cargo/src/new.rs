use std::process::Command;

use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct NewArg {
    #[clap(short, long)]
    snap_name: Option<String>,
    name: String,
}

impl NewArg {
    pub fn execute(self) -> Result<()> {
        let snap_name = if let Some(s) = self.snap_name {
            s
        } else {
            self.name.clone()
        };

        let mut output = Command::new("wasm-pack")
            .arg("new")
            .arg("--template")
            .arg("https://github.com/tiannian/rusnap-template.git")
            .arg(&snap_name)
            .spawn()?;

        output.wait()?;

        Ok(())
    }
}
