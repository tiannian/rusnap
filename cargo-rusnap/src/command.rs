use anyhow::Result;
use clap::Subcommand;

use crate::{build::BuildArg, new::NewArg, serve, start::StartArg};

#[derive(Debug, Subcommand)]
pub enum Rusnap {
    New(NewArg),
    Build(BuildArg),
    Start(StartArg),
    Serve(serve::ServeArg),
}

impl Rusnap {
    pub fn execute(self) -> Result<()> {
        match self {
            Self::New(arg) => {
                arg.execute()?;
            }

            Self::Build(arg) => {
                arg.execute()?;
            }
            Self::Start(arg) => {
                arg.execute()?;
            }
            Self::Serve(arg) => {
                arg.execute()?;
            }
        }

        Ok(())
    }
}
