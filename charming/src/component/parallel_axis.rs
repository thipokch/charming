use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::element::{AxisType, NameLocation};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParallelAxis {
    dim: Option<f64>,

    parallel_index: Option<f64>,

    realtime: Option<bool>,

    #[serde(rename = "type")]
    type_: Option<AxisType>,

    name: Option<String>,

    name_location: Option<NameLocation>,

    name_gap: Option<f64>,

    inverse: Option<bool>,

    max: Option<f64>,

    min: Option<f64>,


    data: Vec<String>,
}

impl ParallelAxis {
    pub fn new() -> Self {
        Self {
            dim: None,
            parallel_index: None,
            realtime: None,
            type_: None,
            name: None,
            name_location: None,
            name_gap: None,
            inverse: None,
            max: None,
            min: None,
            data: vec![],
        }
    }

    pub fn dim<F: Into<f64>>(mut self, dim: F) -> Self {
        self.dim = Some(dim.into());
        self
    }

    pub fn parallel_index<F: Into<f64>>(mut self, parallel_index: F) -> Self {
        self.parallel_index = Some(parallel_index.into());
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }

    pub fn type_<S: Into<AxisType>>(mut self, type_: S) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name_location<E: Into<NameLocation>>(mut self, name_location: E) -> Self {
        self.name_location = Some(name_location.into());
        self
    }

    pub fn name_gap<F: Into<f64>>(mut self, name_gap: F) -> Self {
        self.name_gap = Some(name_gap.into());
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}
