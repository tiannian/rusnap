use anyhow::{anyhow, Result};

use crate::{status::DepInfo, utils};

pub fn execute(info: &DepInfo) -> Result<()> {
    let target_path = utils::get_rusnap_path()?;

    let mut cmd = info.npm_run().ok_or(anyhow!("No npm found"))?;
    let res = cmd.arg("serve").current_dir(&target_path).spawn()?.wait()?;

    if res.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to execute serve"))
    }
}
