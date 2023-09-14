use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::line_style::LineStyle;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitLine {
    show: Option<bool>,

    distance: Option<f64>,

    line_style: Option<LineStyle>,

    length: Option<f64>,
}

impl SplitLine {
    pub fn new() -> Self {
        Self {
            show: None,
            distance: None,
            line_style: None,
            length: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
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

    pub fn length<F: Into<f64>>(mut self, length: F) -> Self {
        self.length = Some(length.into());
        self
    }
}
