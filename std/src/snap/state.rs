use serde::{Deserialize, Serialize};

use crate::Result;

use super::request;

#[derive(Debug, Serialize, Deserialize)]
struct ManageState<D> {
    operation: &'static str,
    #[serde(rename = "newState")]
    state: Option<D>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ManageStateBare {
    operation: &'static str,
}

pub async fn state_update<D>(state: D) -> Result<()>
where
    D: Serialize,
{
    let arg = ManageState {
        operation: "update",
        state: Some(state),
    };

    request("snap_manageState", arg).await?;

    Ok(())
}

pub async fn state_clear() -> Result<()> {
    request("snap_manageState", ManageStateBare { operation: "clear" }).await?;

    Ok(())
}

pub async fn state_get<D>() -> Result<D>
where
    D: for<'de> Deserialize<'de>,
{
    Ok(request("snap_manageState", ManageStateBare { operation: "get" }).await?)
}
