use anyhow::Result;
use clap::Args;

use crate::serve::serve_http;

#[derive(Args, Debug)]
pub struct StartArg {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl StartArg {
    pub fn execute(self) -> Result<()> {
        serve_http(self.port)?;
        Ok(())
    }
}
