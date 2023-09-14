use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::{datatype::CompositeValue, element::Orient};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Value,
    Category,
    Time,
    Log,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleAxis {
    #[serde(rename = "type")]
    type_: Option<Type>,

    name: Option<String>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,

    width: Option<CompositeValue>,

    height: Option<CompositeValue>,

    orient: Option<Orient>,

    inverse: Option<bool>,

    min: Option<String>,

    max: Option<String>,
}

impl SingleAxis {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            orient: None,
            inverse: None,
            min: None,
            max: None,
        }
    }

    pub fn type_(mut self, type_: Type) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
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

    pub fn orient(mut self, orient: Orient) -> Self {
        self.orient = Some(orient);
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn min<S: Into<String>>(mut self, min: S) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<S: Into<String>>(mut self, max: S) -> Self {
        self.max = Some(max.into());
        self
    }
}
