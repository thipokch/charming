use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::datatype::CompositeValue;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionEncode {
    x: Option<CompositeValue>,

    y: Option<CompositeValue>,

    z: Option<CompositeValue>,

    item_name: Option<String>,


    tooltip: Vec<CompositeValue>,
}

impl DimensionEncode {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            item_name: None,
            tooltip: vec![],
        }
    }

    pub fn x<C: Into<CompositeValue>>(mut self, x: C) -> Self {
        self.x = Some(x.into());
        self
    }

    pub fn y<C: Into<CompositeValue>>(mut self, y: C) -> Self {
        self.y = Some(y.into());
        self
    }

    pub fn z<C: Into<CompositeValue>>(mut self, z: C) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn item_name<S: Into<String>>(mut self, item_name: S) -> Self {
        self.item_name = Some(item_name.into());
        self
    }

    pub fn tooltip<S: Into<CompositeValue>>(mut self, tooltip: Vec<S>) -> Self {
        self.tooltip = tooltip.into_iter().map(|s| s.into()).collect();
        self
    }
}
