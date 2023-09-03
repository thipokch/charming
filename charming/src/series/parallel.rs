use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem, Emphasis, LineStyle},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressiveChunkMode {
    Sequential,
    Mod,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parallel {
    #[serde(rename = "type")]
    type_: String,

    id: Option<String>,

    coordinate_system: Option<CoordinateSystem>,

    parallel_index: Option<f64>,

    name: Option<String>,

    color_by: Option<ColorBy>,

    line_style: Option<LineStyle>,

    emphasis: Option<Emphasis>,

    inactive_opacity: Option<f64>,

    active_opacity: Option<f64>,

    realtime: Option<bool>,

    smooth: Option<f64>,

    progressive: Option<f64>,

    progressive_threshold: Option<f64>,

    progressive_chunk_mode: Option<ProgressiveChunkMode>,


    data: DataFrame,
}

impl Parallel {
    pub fn new() -> Self {
        Self {
            type_: "parallel".to_string(),
            id: None,
            coordinate_system: None,
            parallel_index: None,
            name: None,
            color_by: None,
            line_style: None,
            emphasis: None,
            inactive_opacity: None,
            active_opacity: None,
            realtime: None,
            smooth: None,
            progressive: None,
            progressive_threshold: None,
            progressive_chunk_mode: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn parallel_index<F: Into<f64>>(mut self, parallel_index: F) -> Self {
        self.parallel_index = Some(parallel_index.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by<C: Into<ColorBy>>(mut self, color_by: C) -> Self {
        self.color_by = Some(color_by.into());
        self
    }

    pub fn line_style<S: Into<LineStyle>>(mut self, line_style: S) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn inactive_opacity<F: Into<f64>>(mut self, inactive_opacity: F) -> Self {
        self.inactive_opacity = Some(inactive_opacity.into());
        self
    }

    pub fn active_opacity<F: Into<f64>>(mut self, active_opacity: F) -> Self {
        self.active_opacity = Some(active_opacity.into());
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }

    pub fn smooth<F: Into<f64>>(mut self, smooth: F) -> Self {
        self.smooth = Some(smooth.into());
        self
    }

    pub fn progressive<F: Into<f64>>(mut self, progressive: F) -> Self {
        self.progressive = Some(progressive.into());
        self
    }

    pub fn progressive_threshold<F: Into<f64>>(mut self, progressive_threshold: F) -> Self {
        self.progressive_threshold = Some(progressive_threshold.into());
        self
    }

    pub fn progressive_chunk_mode<P: Into<ProgressiveChunkMode>>(
        mut self,
        progressive_chunk_mode: P,
    ) -> Self {
        self.progressive_chunk_mode = Some(progressive_chunk_mode.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
