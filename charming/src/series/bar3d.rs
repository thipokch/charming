use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{CoordinateSystem, DimensionEncode},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar3d {
    #[serde(rename = "type")]
    type_: String,

    name: Option<String>,

    coordinate_system: Option<CoordinateSystem>,

    grid3d_index: Option<CompositeValue>,

    geo3d_index: Option<CompositeValue>,

    globe_index: Option<CompositeValue>,

    shading: Option<String>,

    encode: Option<DimensionEncode>,


    data: DataFrame,
}
impl Bar3d {
    pub fn new() -> Self {
        Self {
            type_: "bar3D".to_string(),
            name: None,
            coordinate_system: None,
            grid3d_index: None,
            geo3d_index: None,
            globe_index: None,
            shading: None,
            encode: None,
            data: DataFrame::new(),
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn grid3d_index<C: Into<CompositeValue>>(mut self, grid3d_index: C) -> Self {
        self.grid3d_index = Some(grid3d_index.into());
        self
    }

    pub fn geo3d_index<C: Into<CompositeValue>>(mut self, geo3d_index: C) -> Self {
        self.geo3d_index = Some(geo3d_index.into());
        self
    }

    pub fn globe_index<C: Into<CompositeValue>>(mut self, globe_index: C) -> Self {
        self.globe_index = Some(globe_index.into());
        self
    }

    pub fn shading<S: Into<String>>(mut self, shading: S) -> Self {
        self.shading = Some(shading.into());
        self
    }

    pub fn encode<D: Into<DimensionEncode>>(mut self, encode: D) -> Self {
        self.encode = Some(encode.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
