use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::{area_style::AreaStyle, border_type::BorderType, color::Color, line_style::LineStyle};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundStyle {
    color: Option<Color>,

    border_color: Option<Color>,

    border_width: Option<f64>,

    border_type: Option<BorderType>,

    border_radius: Option<f64>,

    opacity: Option<f64>,
}

impl BackgroundStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            border_color: None,
            border_width: None,
            border_type: None,
            border_radius: None,
            opacity: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn border_color<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color = Some(border_color.into());
        self
    }

    pub fn border_width<F: Into<f64>>(mut self, border_width: F) -> Self {
        self.border_width = Some(border_width.into());
        self
    }

    pub fn border_type<B: Into<BorderType>>(mut self, border_type: B) -> Self {
        self.border_type = Some(border_type.into());
        self
    }

    pub fn border_radius<F: Into<f64>>(mut self, border_radius: F) -> Self {
        self.border_radius = Some(border_radius.into());
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBackground {
    line_style: Option<LineStyle>,

    area_style: Option<AreaStyle>,
}

impl DataBackground {
    pub fn new() -> Self {
        Self {
            line_style: None,
            area_style: None,
        }
    }

    pub fn line_style<L: Into<LineStyle>>(mut self, line_style: L) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn area_style<A: Into<AreaStyle>>(mut self, area_style: A) -> Self {
        self.area_style = Some(area_style.into());
        self
    }
}
