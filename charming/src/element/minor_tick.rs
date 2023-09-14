use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::line_style::LineStyle;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinorTick {
    show: Option<bool>,

    split_number: Option<f64>,

    length: Option<f64>,

    line_style: Option<LineStyle>,
}

impl MinorTick {
    pub fn new() -> MinorTick {
        MinorTick {
            show: None,
            split_number: None,
            length: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> MinorTick {
        self.show = Some(show);
        self
    }

    pub fn split_number<F: Into<f64>>(mut self, split_number: F) -> MinorTick {
        self.split_number = Some(split_number.into());
        self
    }

    pub fn length<F: Into<f64>>(mut self, length: F) -> MinorTick {
        self.length = Some(length.into());
        self
    }

    pub fn line_style<F: Into<LineStyle>>(mut self, line_style: F) -> MinorTick {
        self.line_style = Some(line_style.into());
        self
    }
}
