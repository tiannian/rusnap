use serde::{Deserialize, Serialize};

use crate::Result;

use super::request;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum UiComponent {
    Panel { children: Vec<UiComponent> },
    Copyable { value: String },
    Divider,
    Heading { value: String },
    Spinner,
    Text { value: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct DialogParams {
    #[serde(rename = "type")]
    ty: &'static str,
    content: UiComponent,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
}

pub async fn dialog_alert(content: UiComponent) -> Result<()> {
    let req = DialogParams {
        ty: "Alert",
        content,
        placeholder: None,
    };

    request("snap_dialog", req).await?;

    Ok(())
}

pub async fn dialog_confirmation(content: UiComponent) -> Result<()> {
    let req = DialogParams {
        ty: "Confirmation",
        content,
        placeholder: None,
    };

    request("snap_dialog", req).await?;

    Ok(())
}

pub async fn dialog_prompt(content: UiComponent, placeholder: &str) -> Result<()> {
    let req = DialogParams {
        ty: "Prompt",
        content,
        placeholder: Some(String::from(placeholder)),
    };

    request("snap_dialog", req).await?;

    Ok(())
}
