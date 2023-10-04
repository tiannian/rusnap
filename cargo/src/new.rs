use std::{fs, path::Path, process::Command};

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
            .arg(&self.name)
            .spawn()?;

        output.wait()?;

        let p = Path::new(&self.name);

        let snap_toml = include_str!("../assets/Snap.toml");
        let snap_config = snap_toml.replace("RUSNAP_NAME", &snap_name);
        fs::write(p.join("Snap.toml"), snap_config)?;

        let build_rs = include_str!("../assets/build.rs");
        fs::write(p.join("build.rs"), build_rs)?;

        let c = include_str!("../assets/icon.svg");
        fs::write(p.join("icon.svg"), c)?;

        Ok(())
    }
}
