use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::element::AxisType;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Axis3D {
    type_: Option<AxisType>,
}

impl Axis3D {
    pub fn new() -> Self {
        Self { type_: None }
    }

    pub fn type_<A: Into<AxisType>>(mut self, type_: A) -> Self {
        self.type_ = Some(type_.into());
        self
    }
}
