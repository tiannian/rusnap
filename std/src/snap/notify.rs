use serde::{Deserialize, Serialize};

use crate::Result;

use super::request;

/// Type of Notify
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotifyType {
    /// Display the notification in the MetaMask UI
    InApp,
    /// Display the notification in the browser
    Native,
}

#[derive(Debug, Serialize, Deserialize)]
struct NotifyParams<'a> {
    #[serde(rename = "type")]
    pub ty: NotifyType,
    pub message: &'a str,
}

/// Displays a notification in MetaMask or natively in the browser.
///
/// Snap Document: [snap_notify](https://docs.metamask.io/snaps/reference/rpc-api/#snap_notify)
pub async fn notify(ty: NotifyType, message: &str) -> Result<()> {
    request("snap_notify", NotifyParams { ty, message }).await
}
