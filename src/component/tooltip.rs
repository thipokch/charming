use serde::Serialize;

use crate::element::{
    axis_pointer::AxisPointer, color::Color, padding::Padding, tooltip_trigger::TooltipTrigger,
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOn {
    Mousemove,
    Click,
    #[serde(rename = "mousemove|click")]
    MousemoveAndClick,
    None,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<TooltipTrigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_on: Option<TriggerOn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_pointer: Option<AxisPointer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<Padding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn trigger<T: Into<TooltipTrigger>>(mut self, trigger: T) -> Self {
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

    pub fn formatter<S: Into<String>>(mut self, formatter: S) -> Self {
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
