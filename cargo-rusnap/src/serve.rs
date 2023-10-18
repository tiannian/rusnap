use anyhow::Result;
use clap::Args;
use tokio::runtime::Runtime;

use crate::utils;

#[derive(Args, Debug)]
pub struct ServeArg {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl ServeArg {
    pub fn execute(self) -> Result<()> {
        serve_http(self.port)?;
        Ok(())
    }
}

pub fn serve_http(port: u16) -> Result<()> {
    let rusnap_dir = utils::get_rusnap_path()?;

    let rt = Runtime::new()?;

    rt.block_on(async move {
        warp::serve(warp::fs::dir(rusnap_dir))
            .run(([0, 0, 0, 0], port))
            .await;
    });

    Ok(())
}
