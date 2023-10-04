use anyhow::{anyhow, Result};
use clap::Subcommand;

use crate::{build::BuildArg, new::NewArg, start, status::DepInfo};

#[derive(Debug, Subcommand)]
pub enum Rusnap {
    Status,
    New(NewArg),
    Build(BuildArg),
    Start,
    Publish,
}

fn status(info: &DepInfo) -> Result<()> {
    if !info.is_right() {
        println!("{:?}", info);

        Err(anyhow!("Failed to check"))
    } else {
        Ok(())
    }
}

impl Rusnap {
    pub fn execute(self) -> Result<()> {
        let info = DepInfo::new()?;

        match self {
            Self::Status => {
                if !info.is_right() {
                    println!("{:?}", info);
                } else {
                    println!("Success {:?}", info);
                }
            }
            Self::New(arg) => {
                status(&info)?;
                arg.execute()?;
            }

            Self::Build(arg) => {
                status(&info)?;
                arg.execute(&info)?;
            }
            Self::Start => {
                start::execute(&info)?;
            }
            _ => {}
        }

        Ok(())
    }
}
