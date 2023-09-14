use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::datatype::CompositeValue;

/// Polar coordinate can be used in scatter and line chart.
#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolarCoordinate {
    id: Option<String>,

    /// The `zlevel` value of all graphical elements in.
    zlevel: Option<f64>,

    /// The `z` value of all graphical elements in.
    z: Option<f64>,

    center: Option<CompositeValue>,

    radius: Option<CompositeValue>,
}

impl PolarCoordinate {
    pub fn new() -> Self {
        Self {
            id: None,
            zlevel: None,
            z: None,
            center: None,
            radius: None,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
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

    pub fn center<C: Into<CompositeValue>>(mut self, center: C) -> Self {
        self.center = Some(center.into());
        self
    }

    pub fn radius<C: Into<CompositeValue>>(mut self, radius: C) -> Self {
        self.radius = Some(radius.into());
        self
    }
}
