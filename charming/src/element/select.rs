use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::{item_style::ItemStyle, label::Label};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    disabled: Option<bool>,

    label: Option<Label>,

    item_style: Option<ItemStyle>,
}

impl Select {
    pub fn new() -> Self {
        Self {
            disabled: None,
            label: None,
            item_style: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }
}
