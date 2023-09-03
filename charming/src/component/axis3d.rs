use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::element::AxisType;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
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
