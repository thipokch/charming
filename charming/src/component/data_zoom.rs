use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::CompositeValue,
    element::{Color, DataBackground, Orient, TextStyle},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Filter,
    WeakFilter,
    Empty,
    None,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataZoomType {
    Inside,
    Slider,
    Select,
}

/// DataZoom component is used for zooming a specific area, which enables user
/// to investigate data in detail, or get an overview of the data, or get rid
/// of outlier points.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataZoom {
    #[serde(rename = "type")]
    type_: Option<DataZoomType>,

    /// Component ID.
    id: Option<String>,

    /// Whether to show the component.
    show: Option<bool>,

    /// Whether to enable real-time view update.
    realtime: Option<bool>,

    /// Background color of the component.
    background_color: Option<Color>,

    /// Style of the data shadow.
    data_background: Option<DataBackground>,

    /// Style of the selected data shadow.
    selected_data_background: Option<DataBackground>,

    /// Color to fill selected area.
    filler_color: Option<Color>,

    /// Color of border.
    border_color: Option<Color>,

    start: Option<f64>,

    end: Option<f64>,

    start_value: Option<f64>,

    end_value: Option<f64>,

    min_span: Option<f64>,

    max_span: Option<f64>,

    min_value_span: Option<f64>,

    max_value_span: Option<f64>,

    orient: Option<Orient>,

    zoom_lock: Option<bool>,

    throttle: Option<f64>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,

    x_axis_index: Option<CompositeValue>,

    y_axis_index: Option<CompositeValue>,

    disabled: Option<bool>,

    radius_axis_index: Option<f64>,

    angle_axis_index: Option<f64>,

    filter_mode: Option<FilterMode>,

    text_style: Option<TextStyle>,

    handle_icon: Option<String>,

    brush_select: Option<bool>,
}

impl DataZoom {
    pub fn new() -> Self {
        Self {
            type_: None,
            id: None,
            show: None,
            realtime: None,
            background_color: None,
            data_background: None,
            selected_data_background: None,
            filler_color: None,
            border_color: None,
            start: None,
            end: None,
            start_value: None,
            end_value: None,
            min_span: None,
            max_span: None,
            min_value_span: None,
            max_value_span: None,
            orient: None,
            zoom_lock: None,
            throttle: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            x_axis_index: None,
            y_axis_index: None,
            disabled: None,
            radius_axis_index: None,
            angle_axis_index: None,
            filter_mode: None,
            text_style: None,
            handle_icon: None,
            brush_select: None,
        }
    }

    pub fn type_<T: Into<DataZoomType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }

    pub fn background_color<C: Into<Color>>(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color.into());
        self
    }

    pub fn data_background<D: Into<DataBackground>>(mut self, data_background: D) -> Self {
        self.data_background = Some(data_background.into());
        self
    }

    pub fn selected_data_background<D: Into<DataBackground>>(
        mut self,
        selected_data_background: D,
    ) -> Self {
        self.selected_data_background = Some(selected_data_background.into());
        self
    }

    pub fn filler_color<C: Into<Color>>(mut self, filler_color: C) -> Self {
        self.filler_color = Some(filler_color.into());
        self
    }

    pub fn border_color<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color = Some(border_color.into());
        self
    }

    pub fn start<F: Into<f64>>(mut self, start: F) -> Self {
        self.start = Some(start.into());
        self
    }

    pub fn end<F: Into<f64>>(mut self, end: F) -> Self {
        self.end = Some(end.into());
        self
    }

    pub fn start_value<F: Into<f64>>(mut self, start_value: F) -> Self {
        self.start_value = Some(start_value.into());
        self
    }

    pub fn end_value<F: Into<f64>>(mut self, end_value: F) -> Self {
        self.end_value = Some(end_value.into());
        self
    }

    pub fn min_span<F: Into<f64>>(mut self, min_span: F) -> Self {
        self.min_span = Some(min_span.into());
        self
    }

    pub fn max_span<F: Into<f64>>(mut self, max_span: F) -> Self {
        self.max_span = Some(max_span.into());
        self
    }

    pub fn min_value_span<F: Into<f64>>(mut self, min_value_span: F) -> Self {
        self.min_value_span = Some(min_value_span.into());
        self
    }

    pub fn max_value_span<F: Into<f64>>(mut self, max_value_span: F) -> Self {
        self.max_value_span = Some(max_value_span.into());
        self
    }

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn zoom_lock(mut self, zoom_lock: bool) -> Self {
        self.zoom_lock = Some(zoom_lock);
        self
    }

    pub fn throttle<F: Into<f64>>(mut self, throttle: F) -> Self {
        self.throttle = Some(throttle.into());
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn x_axis_index<C: Into<CompositeValue>>(mut self, x_axis_index: C) -> Self {
        self.x_axis_index = Some(x_axis_index.into());
        self
    }

    pub fn y_axis_index<C: Into<CompositeValue>>(mut self, y_axis_index: C) -> Self {
        self.y_axis_index = Some(y_axis_index.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn radius_axis_index<F: Into<f64>>(mut self, radius_axis_index: F) -> Self {
        self.radius_axis_index = Some(radius_axis_index.into());
        self
    }

    pub fn angle_axis_index<F: Into<f64>>(mut self, angle_axis_index: F) -> Self {
        self.angle_axis_index = Some(angle_axis_index.into());
        self
    }

    pub fn filter_mode<F: Into<FilterMode>>(mut self, filter_mode: F) -> Self {
        self.filter_mode = Some(filter_mode.into());
        self
    }

    pub fn text_style<T: Into<TextStyle>>(mut self, text_style: T) -> Self {
        self.text_style = Some(text_style.into());
        self
    }

    pub fn handle_icon<S: Into<String>>(mut self, handle_icon: S) -> Self {
        self.handle_icon = Some(handle_icon.into());
        self
    }

    pub fn brush_select(mut self, brush_select: bool) -> Self {
        self.brush_select = Some(brush_select);
        self
    }
}
