use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::CompositeValue,
    element::{Emphasis, ItemStyle, Label},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Treemap {
    #[serde(rename = "type")]
    type_: String,

    id: Option<String>,

    name: Option<String>,

    zlevel: Option<f64>,

    z: Option<f64>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,

    width: Option<CompositeValue>,

    height: Option<CompositeValue>,

    label: Option<Label>,

    item_style: Option<ItemStyle>,

    emphasis: Option<Emphasis>,
}

impl Treemap {
    pub fn new() -> Self {
        Treemap {
            type_: "treemap".to_string(),
            id: None,
            name: None,
            zlevel: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            label: None,
            item_style: None,
            emphasis: None,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn zlevel<F: Into<f64>>(mut self, zlevel: F) -> Self {
        self.zlevel = Some(zlevel.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<C: Into<CompositeValue>>(mut self, width: C) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<C: Into<CompositeValue>>(mut self, height: C) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }
}
