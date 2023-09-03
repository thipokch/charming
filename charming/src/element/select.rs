use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{item_style::ItemStyle, label::Label};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
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
