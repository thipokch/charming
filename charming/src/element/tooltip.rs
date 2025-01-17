use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::element::{AxisPointer, Color, Formatter, Padding};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOn {
    Mousemove,
    Click,
    #[serde(rename = "mousemove|click")]
    MousemoveAndClick,
    None,
}

/// Types of triggering.
#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    Item,
    Axis,
    None,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    trigger: Option<Trigger>,

    trigger_on: Option<TriggerOn>,

    axis_pointer: Option<AxisPointer>,

    formatter: Option<Formatter>,

    position: Option<String>,

    padding: Option<Padding>,

    background_color: Option<Color>,

    border_color: Option<Color>,

    border_width: Option<f64>,
}

impl Tooltip {
    pub fn new() -> Self {
        Self {
            trigger: None,
            trigger_on: None,
            axis_pointer: None,
            formatter: None,
            position: None,
            padding: None,
            background_color: None,
            border_color: None,
            border_width: None,
        }
    }

    pub fn trigger<T: Into<Trigger>>(mut self, trigger: T) -> Self {
        self.trigger = Some(trigger.into());
        self
    }

    pub fn trigger_on<T: Into<TriggerOn>>(mut self, trigger_on: T) -> Self {
        self.trigger_on = Some(trigger_on.into());
        self
    }

    pub fn axis_pointer<A: Into<AxisPointer>>(mut self, axis_pointer: A) -> Self {
        self.axis_pointer = Some(axis_pointer.into());
        self
    }

    pub fn formatter<F: Into<Formatter>>(mut self, formatter: F) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn position<S: Into<String>>(mut self, position: S) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn background_color<C: Into<Color>>(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color.into());
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
}
