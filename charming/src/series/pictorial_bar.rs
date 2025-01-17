use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::{
    datatype::DataFrame,
    element::{
        ColorBy, CoordinateSystem, Cursor, Emphasis, ItemStyle, Label, LabelLayout, LabelLine,
    },
};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PictorialBar {
    id: Option<String>,

    name: Option<String>,

    color_by: Option<ColorBy>,

    legend_hover_link: Option<bool>,

    coordinate_system: Option<CoordinateSystem>,

    x_axis_index: Option<f64>,

    y_axis_index: Option<f64>,

    cursor: Option<Cursor>,

    label: Option<Label>,

    label_line: Option<LabelLine>,

    label_layout: Option<LabelLayout>,

    item_style: Option<ItemStyle>,

    emphasis: Option<Emphasis>,

    symbol_clip: Option<bool>,

    symbol_bounding_data: Option<f64>,


    data: Vec<DataFrame>,
}

impl PictorialBar {
    pub fn new() -> Self {
        PictorialBar {
            id: None,
            name: None,
            color_by: None,
            legend_hover_link: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            cursor: None,
            label: None,
            label_line: None,
            label_layout: None,
            item_style: None,
            emphasis: None,
            symbol_clip: None,
            symbol_bounding_data: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
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

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn x_axis_index<F: Into<f64>>(mut self, x_axis_index: F) -> Self {
        self.x_axis_index = Some(x_axis_index.into());
        self
    }

    pub fn y_axis_index<F: Into<f64>>(mut self, y_axis_index: F) -> Self {
        self.y_axis_index = Some(y_axis_index.into());
        self
    }

    pub fn cursor<C: Into<Cursor>>(mut self, cursor: C) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn label_line<L: Into<LabelLine>>(mut self, label_line: L) -> Self {
        self.label_line = Some(label_line.into());
        self
    }

    pub fn label_layout<L: Into<LabelLayout>>(mut self, label_layout: L) -> Self {
        self.label_layout = Some(label_layout.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn symbol_clip(mut self, symbol_clip: bool) -> Self {
        self.symbol_clip = Some(symbol_clip);
        self
    }

    pub fn symbol_bounding_data<F: Into<f64>>(mut self, symbol_bounding_data: F) -> Self {
        self.symbol_bounding_data = Some(symbol_bounding_data.into());
        self
    }

    pub fn data<D: Into<DataFrame>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
