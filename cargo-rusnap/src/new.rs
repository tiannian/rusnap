use anyhow::Result;
use clap::Args;
use wasm_pack::{
    command::{run_wasm_pack, Command},
    install::InstallMode,
};

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

        let template = "https://github.com/tiannian/rusnap-template.git".into();

        let command = Command::Generate {
            name: snap_name,
            template,
            mode: InstallMode::Normal,
        };

        run_wasm_pack(command)?;

        Ok(())
    }
}
