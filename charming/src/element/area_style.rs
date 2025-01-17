use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::color::Color;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginPosition {
    Auto,
    Start,
    End,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaStyle {
    color: Option<Color>,

    origin: Option<OriginPosition>,

    opacity: Option<f64>,
}

impl AreaStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            origin: None,
            opacity: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn origin<O: Into<OriginPosition>>(mut self, origin: O) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }
}
