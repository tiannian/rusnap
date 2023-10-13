use serde::{Deserialize, Serialize};

use crate::Result;

use super::request;

/// Snap custom UI Component.
///
/// Snap Document: [Components](https://docs.metamask.io/snaps/how-to/use-custom-ui/#components)
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

/// Helper function to create `UiComponent`.
pub mod ui {
    use super::UiComponent;

    /// A panel, which can be used as a container for other components.
    ///
    /// Snap Document: [panel](https://docs.metamask.io/snaps/how-to/use-custom-ui/#panel)
    pub fn panel<T>(children: T) -> UiComponent
    where
        T: Into<Vec<UiComponent>>,
    {
        UiComponent::Panel {
            children: children.into(),
        }
    }

    /// A read-only text field with a copy-to-clipboard shortcut.
    ///
    /// Snap Document: [copyable](https://docs.metamask.io/snaps/how-to/use-custom-ui/#copyable)
    pub fn copyable<T>(value: T) -> UiComponent
    where
        T: Into<String>,
    {
        UiComponent::Copyable {
            value: value.into(),
        }
    }

    /// A horizontal divider.
    ///
    /// Snap Document: [divider](https://docs.metamask.io/snaps/how-to/use-custom-ui/#divider)
    pub fn divider() -> UiComponent {
        UiComponent::Divider
    }

    /// A heading.
    ///
    /// Snap Document: [heading](https://docs.metamask.io/snaps/how-to/use-custom-ui/#heading)
    pub fn heading<T>(value: T) -> UiComponent
    where
        T: Into<String>,
    {
        UiComponent::Heading {
            value: value.into(),
        }
    }

    /// A loading indicator.
    ///
    /// Snap Document: [spinner](https://docs.metamask.io/snaps/how-to/use-custom-ui/#spinner)
    pub fn spinner() -> UiComponent {
        UiComponent::Spinner
    }

    /// Text
    ///
    /// Snap Document: [text](https://docs.metamask.io/snaps/how-to/use-custom-ui/#text)
    pub fn text<T>(value: T) -> UiComponent
    where
        T: Into<String>,
    {
        UiComponent::Text {
            value: value.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DialogParams {
    #[serde(rename = "type")]
    ty: &'static str,
    content: UiComponent,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
}

/// Create an alert dialog.
///
/// Snap Document: [Alert dialog](https://docs.metamask.io/snaps/reference/rpc-api/#alert-dialog)
pub async fn alert(content: UiComponent) -> Result<()> {
    let req = DialogParams {
        ty: "alert",
        content,
        placeholder: None,
    };

    request("snap_dialog", req).await?;

    Ok(())
}

/// Create a confirmation dialog.
///
/// Snap Document: [Confirmation dialog](https://docs.metamask.io/snaps/reference/rpc-api/#confirmation-dialog)
pub async fn confirm(content: UiComponent) -> Result<bool> {
    let req = DialogParams {
        ty: "confirmation",
        content,
        placeholder: None,
    };

    Ok(request("snap_dialog", req).await?)
}

/// Create a prompt dialog
///
/// Snap Document: [Prompt dialog](https://docs.metamask.io/snaps/reference/rpc-api/#prompt-dialog)
pub async fn prompt(content: UiComponent, placeholder: &str) -> Result<Option<String>> {
    let req = DialogParams {
        ty: "prompt",
        content,
        placeholder: Some(String::from(placeholder)),
    };

    Ok(request("snap_dialog", req).await?)
}
