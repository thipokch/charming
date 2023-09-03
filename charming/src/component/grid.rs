use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::CompositeValue,
    element::{Color, Padding, TextStyle, Trigger},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridTooltip {
    /// Whether to show the tooltip component.
    show: Option<bool>,

    /// Type of triggering.
    trigger: Option<Trigger>,

    /// The position of the tooltip's floating layer, which would follow the
    /// position of mouse by default.
    position: Option<(String, String)>,

    /// The content formatter of tooltip's floating layer.
    formatter: Option<String>,

    /// The value formatter of tooltip's floating layer.
    value_formatter: Option<String>,

    /// The background color of tooltip's floating layer.
    background_color: Option<Color>,

    /// The border color of tooltip's floating layer.
    border_color: Option<Color>,

    /// The border width of tooltip's floating layer.
    border_width: Option<f64>,

    /// The floating layer of tooltip space around content.
    padding: Option<Padding>,

    /// Text style of tooltip's floating layer.
    text_style: Option<TextStyle>,

    /// Extra CSS style for the tooltip's floating layer.
    extra_css_text: Option<String>,
}

impl GridTooltip {
    pub fn new() -> Self {
        Self {
            show: None,
            trigger: None,
            position: None,
            formatter: None,
            value_formatter: None,
            background_color: None,
            border_color: None,
            border_width: None,
            padding: None,
            text_style: None,
            extra_css_text: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn trigger<T: Into<Trigger>>(mut self, trigger: T) -> Self {
        self.trigger = Some(trigger.into());
        self
    }

    pub fn position<S: Into<String>>(mut self, position: (S, S)) -> Self {
        self.position = Some((position.0.into(), position.1.into()));
        self
    }

    pub fn formatter<S: Into<String>>(mut self, formatter: S) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn value_formatter<S: Into<String>>(mut self, value_formatter: S) -> Self {
        self.value_formatter = Some(value_formatter.into());
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

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn text_style<T: Into<TextStyle>>(mut self, text_style: T) -> Self {
        self.text_style = Some(text_style.into());
        self
    }

    pub fn extra_css_text<S: Into<String>>(mut self, extra_css_text: S) -> Self {
        self.extra_css_text = Some(extra_css_text.into());
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    /// Component ID.
    id: Option<String>,

    /// Whether to show the grid in rectangular coordinate.
    show: Option<bool>,

    /// The `zlevel` value of all graphical elements in.
    zlevel: Option<f64>,

    /// The `z` value of all graphical elements in.
    z: Option<f64>,

    /// Distance between grid component and the left side of the container.
    left: Option<CompositeValue>,

    /// Distance between grid component and the top side of the container.
    top: Option<CompositeValue>,

    /// Distance between grid component and the right side of the container.
    right: Option<CompositeValue>,

    /// Distance between grid component and the bottom side of the container.
    bottom: Option<CompositeValue>,

    /// Width of grid component.
    width: Option<CompositeValue>,

    /// Height of grid component.
    height: Option<CompositeValue>,

    /// Whether the grid region contains axis tick label of axis.
    contain_label: Option<bool>,

    /// Background color of grid, which is transparent by default.
    background_color: Option<Color>,

    /// Border color of grid.
    border_color: Option<Color>,

    /// Border width of grid.
    border_width: Option<f64>,

    /// Size of shadow blue.
    shadow_blur: Option<f64>,

    /// Shadow color.
    shadow_color: Option<Color>,

    /// Offset distance on the horizontal direction of shadow.
    shadow_offset_x: Option<f64>,

    /// Offset distance on the vertical direction of shadow.
    shadow_offset_y: Option<f64>,

    /// Tooltip settings in the grid.
    tooltip: Option<GridTooltip>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            id: None,
            show: None,
            zlevel: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            contain_label: None,
            background_color: None,
            border_color: None,
            border_width: None,
            shadow_blur: None,
            shadow_color: None,
            shadow_offset_x: None,
            shadow_offset_y: None,
            tooltip: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn zlevel<F: Into<f64>>(mut self, zlevel: F) -> Self {
        self.zlevel = Some(zlevel.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<C: Into<CompositeValue>>(mut self, width: C) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<C: Into<CompositeValue>>(mut self, height: C) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn contain_label(mut self, contain_label: bool) -> Self {
        self.contain_label = Some(contain_label);
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

    pub fn shadow_blur<F: Into<f64>>(mut self, shadow_blur: F) -> Self {
        self.shadow_blur = Some(shadow_blur.into());
        self
    }

    pub fn shadow_color<C: Into<Color>>(mut self, shadow_color: C) -> Self {
        self.shadow_color = Some(shadow_color.into());
        self
    }

    pub fn shadow_offset_x<F: Into<f64>>(mut self, shadow_offset_x: F) -> Self {
        self.shadow_offset_x = Some(shadow_offset_x.into());
        self
    }

    pub fn shadow_offset_y<F: Into<f64>>(mut self, shadow_offset_y: F) -> Self {
        self.shadow_offset_y = Some(shadow_offset_y.into());
        self
    }

    pub fn tooltip<T: Into<GridTooltip>>(mut self, tooltip: T) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}
