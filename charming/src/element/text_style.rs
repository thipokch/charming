use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::color::Color;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    color: Option<Color>,

    font_style: Option<String>,

    font_weight: Option<String>,

    font_family: Option<String>,

    font_size: Option<f64>,

    line_height: Option<f64>,

    align: Option<String>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            font_style: None,
            font_weight: None,
            font_family: None,
            font_size: None,
            line_height: None,
            align: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn font_style<S: Into<String>>(mut self, font_style: S) -> Self {
        self.font_style = Some(font_style.into());
        self
    }

    pub fn font_weight<S: Into<String>>(mut self, font_weight: S) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }

    pub fn font_family<S: Into<String>>(mut self, font_family: S) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    pub fn font_size<F: Into<f64>>(mut self, font_size: F) -> Self {
        self.font_size = Some(font_size.into());
        self
    }

    pub fn line_height<F: Into<f64>>(mut self, line_height: F) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn align<S: Into<String>>(mut self, align: S) -> Self {
        self.align = Some(align.into());
        self
    }
}
