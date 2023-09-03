use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint, Dimension},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, ItemStyle, LabelLayout, LabelLine, RawString,
    },
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    type_: String,

    id: Option<String>,

    name: Option<String>,

    color_by: Option<ColorBy>,

    legend_hover_link: Option<bool>,

    coordinate_system: Option<CoordinateSystem>,

    x_axis_index: Option<CompositeValue>,

    y_axis_index: Option<CompositeValue>,

    polar_index: Option<CompositeValue>,

    geo_index: Option<CompositeValue>,

    calendar_index: Option<CompositeValue>,

    render_item: Option<RawString>,

    item_style: Option<ItemStyle>,

    label_line: Option<LabelLine>,

    label_layout: Option<LabelLayout>,

    selected_mode: Option<bool>,


    dimensions: Vec<Dimension>,

    encode: Option<DimensionEncode>,


    data: DataFrame,
}

impl Custom {
    pub fn new() -> Self {
        Self {
            type_: "custom".to_string(),
            id: None,
            name: None,
            color_by: None,
            legend_hover_link: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            polar_index: None,
            geo_index: None,
            calendar_index: None,
            render_item: None,
            item_style: None,
            label_line: None,
            label_layout: None,
            selected_mode: None,
            dimensions: vec![],
            encode: None,
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

    pub fn x_axis_index<C: Into<CompositeValue>>(mut self, x_axis_index: C) -> Self {
        self.x_axis_index = Some(x_axis_index.into());
        self
    }

    pub fn y_axis_index<C: Into<CompositeValue>>(mut self, y_axis_index: C) -> Self {
        self.y_axis_index = Some(y_axis_index.into());
        self
    }

    pub fn polar_index<C: Into<CompositeValue>>(mut self, polar_index: C) -> Self {
        self.polar_index = Some(polar_index.into());
        self
    }

    pub fn geo_index<C: Into<CompositeValue>>(mut self, geo_index: C) -> Self {
        self.geo_index = Some(geo_index.into());
        self
    }

    pub fn calendar_index<C: Into<CompositeValue>>(mut self, calendar_index: C) -> Self {
        self.calendar_index = Some(calendar_index.into());
        self
    }

    pub fn render_item<R: Into<RawString>>(mut self, render_item: R) -> Self {
        self.render_item = Some(render_item.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
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

    pub fn selected_mode(mut self, selected_mode: bool) -> Self {
        self.selected_mode = Some(selected_mode);
        self
    }

    pub fn dimensions<D: Into<Dimension>>(mut self, dimensions: Vec<D>) -> Self {
        self.dimensions = dimensions.into_iter().map(|d| d.into()).collect();
        self
    }

    pub fn encode<E: Into<DimensionEncode>>(mut self, encode: E) -> Self {
        self.encode = Some(encode.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
