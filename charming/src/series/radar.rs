use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{AreaStyle, ColorBy, Emphasis, LineStyle, Symbol, Tooltip},
};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radar {
    area_style: Option<AreaStyle>,

    color_by: Option<ColorBy>,


    data: DataFrame,

    id: Option<String>,

    name: Option<String>,

    radar_index: Option<f64>,

    symbol: Option<Symbol>,

    symbol_keep_aspect: Option<bool>,

    symbol_rotate: Option<f64>,

    symbol_size: Option<f64>,

    tooltip: Option<Tooltip>,

    line_style: Option<LineStyle>,

    emphasis: Option<Emphasis>,
}

impl Radar {
    pub fn new() -> Self {
        Self {
            area_style: None,
            color_by: None,
            data: vec![],
            id: None,
            name: None,
            radar_index: None,
            symbol: None,
            symbol_keep_aspect: None,
            symbol_rotate: None,
            symbol_size: None,
            tooltip: None,
            line_style: None,
            emphasis: None,
        }
    }

    pub fn area_style<A: Into<AreaStyle>>(mut self, area_style: A) -> Self {
        self.area_style = Some(area_style.into());
        self
    }

    pub fn color_by<C: Into<ColorBy>>(mut self, color_by: C) -> Self {
        self.color_by = Some(color_by.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn radar_index<F: Into<f64>>(mut self, radar_index: F) -> Self {
        self.radar_index = Some(radar_index.into());
        self
    }

    pub fn symbol<S: Into<Symbol>>(mut self, symbol: S) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn symbol_keep_aspect(mut self, symbol_keep_aspect: bool) -> Self {
        self.symbol_keep_aspect = Some(symbol_keep_aspect);
        self
    }

    pub fn symbol_rotate<F: Into<f64>>(mut self, symbol_rotate: F) -> Self {
        self.symbol_rotate = Some(symbol_rotate.into());
        self
    }

    pub fn symbol_size<F: Into<f64>>(mut self, symbol_size: F) -> Self {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn tooltip<T: Into<Tooltip>>(mut self, tooltip: T) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn line_style<L: Into<LineStyle>>(mut self, line_style: L) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }
}
