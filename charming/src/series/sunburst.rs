use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::element::{Emphasis, ItemStyle, Label, Sort};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunburstLevel {
    r0: Option<String>,

    r: Option<String>,

    item_style: Option<ItemStyle>,

    label: Option<Label>,
}

impl SunburstLevel {
    pub fn new() -> Self {
        Self {
            r0: None,
            r: None,
            item_style: None,
            label: None,
        }
    }

    pub fn r0<S: Into<String>>(mut self, r0: S) -> Self {
        self.r0 = Some(r0.into());
        self
    }

    pub fn r<S: Into<String>>(mut self, r: S) -> Self {
        self.r = Some(r.into());
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunburstNode {
    name: String,

    value: Option<f64>,

    #[serde(skip_deserializing)]
    item_style: Option<ItemStyle>,


    children: Vec<SunburstNode>,
}

impl SunburstNode {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            value: None,
            item_style: None,
            children: vec![],
        }
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn children(mut self, children: Vec<SunburstNode>) -> Self {
        self.children = children;
        self
    }
}

impl From<&str> for SunburstNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<(&str, f64)> for SunburstNode {
    fn from((name, value): (&str, f64)) -> Self {
        Self::new(name).value(value)
    }
}

impl From<(&str, f64, &str)> for SunburstNode {
    fn from((name, value, color): (&str, f64, &str)) -> Self {
        Self::new(name)
            .value(value)
            .item_style(ItemStyle::new().color(color))
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sunburst {
    id: Option<String>,

    name: Option<String>,

    z_level: Option<u64>,

    z: Option<u64>,

    center: Option<(String, String)>,

    radius: Option<(String, String)>,

    emphasis: Option<Emphasis>,

    sort: Option<Sort>,


    levels: Vec<SunburstLevel>,


    data: Vec<SunburstNode>,
}

impl Sunburst {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            z_level: None,
            z: None,
            center: None,
            radius: None,
            emphasis: None,
            sort: None,
            levels: vec![],
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

    pub fn z_level(mut self, z_level: u64) -> Self {
        self.z_level = Some(z_level);
        self
    }

    pub fn z(mut self, z: u64) -> Self {
        self.z = Some(z);
        self
    }

    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn radius<S: Into<String>>(mut self, radius: (S, S)) -> Self {
        self.radius = Some((radius.0.into(), radius.1.into()));
        self
    }

    pub fn emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphasis = Some(emphasis);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn levels(mut self, levels: Vec<SunburstLevel>) -> Self {
        self.levels = levels;
        self
    }

    pub fn data(mut self, data: Vec<SunburstNode>) -> Self {
        self.data = data;
        self
    }
}
