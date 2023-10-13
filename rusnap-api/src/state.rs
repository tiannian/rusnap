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

/// Update state.
///
/// This function will store data into disk.
///
/// Snap Document: [snap_manageState](https://docs.metamask.io/snaps/reference/rpc-api/#snap_managestate)
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

/// Clear state
///
/// This function will clear data in disk.
///
/// Snap Document: [snap_manageState](https://docs.metamask.io/snaps/reference/rpc-api/#snap_managestate)
pub async fn state_clear() -> Result<()> {
    request("snap_manageState", ManageStateBare { operation: "clear" }).await?;

    Ok(())
}

/// Get state
///
/// This function will get data in disk.
///
/// Snap Document: [snap_manageState](https://docs.metamask.io/snaps/reference/rpc-api/#snap_managestate)
pub async fn state_get<D>() -> Result<D>
where
    D: for<'de> Deserialize<'de>,
{
    Ok(request("snap_manageState", ManageStateBare { operation: "get" }).await?)
}
