use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::element::{ColorBy, CoordinateSystem};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Boxplot {
    id: Option<String>,

    name: Option<String>,

    coordinate_system: Option<CoordinateSystem>,

    color_by: Option<ColorBy>,

    legend_hover_link: Option<bool>,

    hover_animation: Option<bool>,

    dataset_index: Option<u64>,
}

impl Boxplot {
    pub fn new() -> Self {
        Boxplot {
            id: None,
            name: None,
            coordinate_system: None,
            color_by: None,
            legend_hover_link: None,
            hover_animation: None,
            dataset_index: None,
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

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
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

    pub fn hover_animation(mut self, hover_animation: bool) -> Self {
        self.hover_animation = Some(hover_animation);
        self
    }

    pub fn dataset_index(mut self, dataset_index: u64) -> Self {
        self.dataset_index = Some(dataset_index);
        self
    }
}
