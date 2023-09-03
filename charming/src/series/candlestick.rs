use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    #[serde(rename = "type")]
    type_: String,

    id: Option<String>,

    name: Option<String>,

    coordiate_system: Option<CoordinateSystem>,

    color_by: Option<ColorBy>,

    legend_hover_link: Option<bool>,


    data: DataFrame,
}

impl Candlestick {
    pub fn new() -> Self {
        Self {
            type_: "candlestick".to_string(),
            id: None,
            name: None,
            coordiate_system: None,
            color_by: None,
            legend_hover_link: None,
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

    pub fn coordiate_system<C: Into<CoordinateSystem>>(mut self, coordiate_system: C) -> Self {
        self.coordiate_system = Some(coordiate_system.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
