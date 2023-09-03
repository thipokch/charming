use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::color::Color;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineStyleType {
    Solid,
    Dashed,
    Dotted,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    color: Option<Color>,

    width: Option<f64>,

    #[serde(rename = "type")]
    type_: Option<LineStyleType>,

    opacity: Option<f64>,

    curveness: Option<f64>,
}

impl LineStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            width: None,
            type_: None,
            opacity: None,
            curveness: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn type_<L: Into<LineStyleType>>(mut self, type_: L) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }

    pub fn curveness<F: Into<f64>>(mut self, curveness: F) -> Self {
        self.curveness = Some(curveness.into());
        self
    }
}
