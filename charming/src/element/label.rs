use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::{color::Color, line_style::LineStyle, Formatter};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LabelPosition {
    Top,
    Left,
    Right,
    Bottom,
    Inside,
    InsideLeft,
    InsideRight,
    InsideTop,
    InsideBottom,
    InsideTopLeft,
    InsideBottomLeft,
    InsideTopRight,
    InsideBottomRight,
    Start,
    Outside,
    Middle,
    Center,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelAlign {
    Left,
    Center,
    Right,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelVerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    show: Option<bool>,

    position: Option<LabelPosition>,

    distance: Option<f64>,

    rotate: Option<String>,

    offset: Option<(f64, f64)>,

    formatter: Option<Formatter>,

    color: Option<Color>,

    font_size: Option<f64>,

    font_weight: Option<String>,

    padding: Option<(f64, f64, f64, f64)>,

    align: Option<LabelAlign>,

    vertical_align: Option<LabelVerticalAlign>,

    silent: Option<bool>,

    background_color: Option<Color>,

    border_color: Option<Color>,

    border_width: Option<f64>,

    shadow_blur: Option<f64>,

    shadow_offset_x: Option<f64>,

    shadow_offset_y: Option<f64>,
}

impl Label {
    pub fn new() -> Self {
        Self {
            show: None,
            position: None,
            distance: None,
            rotate: None,
            offset: None,
            formatter: None,
            color: None,
            font_size: None,
            font_weight: None,
            padding: None,
            align: None,
            vertical_align: None,
            silent: None,
            background_color: None,
            border_color: None,
            border_width: None,
            shadow_blur: None,
            shadow_offset_x: None,
            shadow_offset_y: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn position<P: Into<LabelPosition>>(mut self, position: P) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn distance<F: Into<f64>>(mut self, distance: F) -> Self {
        self.distance = Some(distance.into());
        self
    }

    pub fn rotate<S: Into<String>>(mut self, rotate: S) -> Self {
        self.rotate = Some(rotate.into());
        self
    }

    pub fn offset<F: Into<f64>>(mut self, offset: (F, F)) -> Self {
        self.offset = Some((offset.0.into(), offset.1.into()));
        self
    }

    pub fn formatter<F: Into<Formatter>>(mut self, formatter: F) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn font_size<F: Into<f64>>(mut self, font_size: F) -> Self {
        self.font_size = Some(font_size.into());
        self
    }

    pub fn font_weight<S: Into<String>>(mut self, font_weight: S) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }

    pub fn padding<F: Into<f64>>(mut self, padding: (F, F, F, F)) -> Self {
        self.padding = Some((
            padding.0.into(),
            padding.1.into(),
            padding.2.into(),
            padding.3.into(),
        ));
        self
    }

    pub fn align<A: Into<LabelAlign>>(mut self, align: A) -> Self {
        self.align = Some(align.into());
        self
    }

    pub fn vertical_align<V: Into<LabelVerticalAlign>>(mut self, vertical_align: V) -> Self {
        self.vertical_align = Some(vertical_align.into());
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
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

    pub fn shadow_offset_x<F: Into<f64>>(mut self, shadow_offset_x: F) -> Self {
        self.shadow_offset_x = Some(shadow_offset_x.into());
        self
    }

    pub fn shadow_offset_y<F: Into<f64>>(mut self, shadow_offset_y: F) -> Self {
        self.shadow_offset_y = Some(shadow_offset_y.into());
        self
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelLine {
    show: Option<bool>,

    show_above: Option<bool>,

    length: Option<f64>,

    smooth: Option<bool>,

    min_turn_angle: Option<f64>,

    line_style: Option<LineStyle>,
}

impl LabelLine {
    pub fn new() -> Self {
        Self {
            show: None,
            show_above: None,
            length: None,
            smooth: None,
            min_turn_angle: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn show_above(mut self, show_above: bool) -> Self {
        self.show_above = Some(show_above);
        self
    }

    pub fn length<F: Into<f64>>(mut self, length: F) -> Self {
        self.length = Some(length.into());
        self
    }

    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = Some(smooth);
        self
    }

    pub fn min_turn_angle<F: Into<f64>>(mut self, min_turn_angle: F) -> Self {
        self.min_turn_angle = Some(min_turn_angle.into());
        self
    }

    pub fn line_style<S: Into<LineStyle>>(mut self, line_style: S) -> Self {
        self.line_style = Some(line_style.into());
        self
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelLayout {
    hide_overlap: Option<bool>,

    overlap: Option<String>,

    rotate: Option<f64>,
}

impl LabelLayout {
    pub fn new() -> Self {
        Self {
            hide_overlap: None,
            overlap: None,
            rotate: None,
        }
    }

    pub fn hide_overlap(mut self, hide_overlap: bool) -> Self {
        self.hide_overlap = Some(hide_overlap);
        self
    }

    pub fn overlap<S: Into<String>>(mut self, overlap: S) -> Self {
        self.overlap = Some(overlap.into());
        self
    }

    pub fn rotate<F: Into<f64>>(mut self, rotate: F) -> Self {
        self.rotate = Some(rotate.into());
        self
    }
}
