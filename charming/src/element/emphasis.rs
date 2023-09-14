use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::{item_style::ItemStyle, AreaStyle, Label};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmphasisFocus {
    None,
    #[serde(rename = "self")]
    Self_,
    Series,
    Ancestor,
    Descendant,
    Relative,
    Adjacency,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emphasis {
    focus: Option<EmphasisFocus>,

    item_style: Option<ItemStyle>,

    area_style: Option<AreaStyle>,

    label: Option<Label>,
}

impl Emphasis {
    pub fn new() -> Self {
        Self {
            focus: None,
            item_style: None,
            area_style: None,
            label: None,
        }
    }

    pub fn focus<E: Into<EmphasisFocus>>(mut self, emphasis: E) -> Self {
        self.focus = Some(emphasis.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn area_style<A: Into<AreaStyle>>(mut self, area_style: A) -> Self {
        self.area_style = Some(area_style.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }
}
