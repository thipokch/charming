use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::{color::Color, Formatter};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisLabel {
    show: Option<bool>,

    distance: Option<f64>,

    font_size: Option<f64>,

    color: Option<Color>,

    formatter: Option<Formatter>,

    rotate: Option<f64>,

    interval: Option<f64>,
}

impl AxisLabel {
    pub fn new() -> Self {
        Self {
            show: None,
            distance: None,
            font_size: None,
            color: None,
            formatter: None,
            rotate: None,
            interval: None,
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

    pub fn font_size<F: Into<f64>>(mut self, font_size: F) -> Self {
        self.font_size = Some(font_size.into());
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn formatter<F: Into<Formatter>>(mut self, formatter: F) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn rotate<F: Into<f64>>(mut self, rotate: F) -> Self {
        self.rotate = Some(rotate.into());
        self
    }

    pub fn interval<F: Into<f64>>(mut self, interval: F) -> Self {
        self.interval = Some(interval.into());
        self
    }
}
