use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::element::{AxisLabel, AxisLine, AxisType, BoundaryGap, NameLocation, TextStyle};

/// Radius axis in polar coordinate.
#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadiusAxis {
    id: Option<String>,

    polar_index: Option<f64>,

    type_: Option<AxisType>,

    name: Option<String>,

    name_location: Option<NameLocation>,

    name_text_style: Option<TextStyle>,

    name_gap: Option<f64>,

    name_rotation: Option<f64>,

    inverse: Option<bool>,

    boundary_gap: Option<BoundaryGap>,

    min: Option<f64>,

    max: Option<f64>,

    scale: Option<bool>,

    split_number: Option<f64>,

    min_interval: Option<f64>,

    max_interval: Option<f64>,

    interval: Option<f64>,

    log_base: Option<f64>,

    axis_label: Option<AxisLabel>,

    axis_line: Option<AxisLine>,


    data: Vec<String>,
}

impl RadiusAxis {
    pub fn new() -> Self {
        Self {
            id: None,
            polar_index: None,
            type_: None,
            name: None,
            name_location: None,
            name_text_style: None,
            name_gap: None,
            name_rotation: None,
            inverse: None,
            boundary_gap: None,
            min: None,
            max: None,
            scale: None,
            split_number: None,
            min_interval: None,
            max_interval: None,
            interval: None,
            log_base: None,
            axis_label: None,
            axis_line: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn polar_index<F: Into<f64>>(mut self, polar_index: F) -> Self {
        self.polar_index = Some(polar_index.into());
        self
    }

    pub fn type_<T: Into<AxisType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name_location<L: Into<NameLocation>>(mut self, name_location: L) -> Self {
        self.name_location = Some(name_location.into());
        self
    }

    pub fn name_text_style<T: Into<TextStyle>>(mut self, name_text_style: T) -> Self {
        self.name_text_style = Some(name_text_style.into());
        self
    }

    pub fn name_gap<F: Into<f64>>(mut self, name_gap: F) -> Self {
        self.name_gap = Some(name_gap.into());
        self
    }

    pub fn name_rotation<F: Into<f64>>(mut self, name_rotation: F) -> Self {
        self.name_rotation = Some(name_rotation.into());
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn boundary_gap<B: Into<BoundaryGap>>(mut self, boundary_gap: B) -> Self {
        self.boundary_gap = Some(boundary_gap.into());
        self
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn scale(mut self, scale: bool) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn split_number<F: Into<f64>>(mut self, split_number: F) -> Self {
        self.split_number = Some(split_number.into());
        self
    }

    pub fn min_interval<F: Into<f64>>(mut self, min_interval: F) -> Self {
        self.min_interval = Some(min_interval.into());
        self
    }

    pub fn max_interval<F: Into<f64>>(mut self, max_interval: F) -> Self {
        self.max_interval = Some(max_interval.into());
        self
    }

    pub fn interval<F: Into<f64>>(mut self, interval: F) -> Self {
        self.interval = Some(interval.into());
        self
    }

    pub fn log_base<F: Into<f64>>(mut self, log_base: F) -> Self {
        self.log_base = Some(log_base.into());
        self
    }

    pub fn axis_label<A: Into<AxisLabel>>(mut self, axis_label: A) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    pub fn axis_line<A: Into<AxisLine>>(mut self, axis_line: A) -> Self {
        self.axis_line = Some(axis_line.into());
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}
