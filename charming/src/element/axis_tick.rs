use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::line_style::LineStyle;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisTick {
    show: Option<bool>,

    split_number: Option<f64>,

    length: Option<f64>,

    distance: Option<f64>,

    line_style: Option<LineStyle>,
}

impl AxisTick {
    pub fn new() -> Self {
        Self {
            show: None,
            split_number: None,
            length: None,
            distance: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn split_number<F: Into<f64>>(mut self, split_number: F) -> Self {
        self.split_number = Some(split_number.into());
        self
    }

    pub fn length<F: Into<f64>>(mut self, length: F) -> Self {
        self.length = Some(length.into());
        self
    }

    pub fn distance<F: Into<f64>>(mut self, distance: F) -> Self {
        self.distance = Some(distance.into());
        self
    }

    pub fn line_style<S: Into<LineStyle>>(mut self, line_style: S) -> Self {
        self.line_style = Some(line_style.into());
        self
    }
}
