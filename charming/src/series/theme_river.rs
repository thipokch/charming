use serde::{ser::SerializeSeq, Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::CompositeValue,
    element::{BoundaryGap, ColorBy, CoordinateSystem, Label},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ThemeRiverData {
    date: CompositeValue,
    value: CompositeValue,
    name: CompositeValue,
}

impl ThemeRiverData {
    pub fn new<D, V, N>(date: D, value: V, name: N) -> Self
    where
        D: Into<CompositeValue>,
        V: Into<CompositeValue>,
        N: Into<CompositeValue>,
    {
        Self {
            date: date.into(),
            value: value.into(),
            name: name.into(),
        }
    }
}

impl Serialize for ThemeRiverData {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_seq(None)?;
        s.serialize_element(&self.date)?;
        s.serialize_element(&self.value)?;
        s.serialize_element(&self.name)?;
        s.end()
    }
}

impl<D, V, N> From<(D, V, N)> for ThemeRiverData
where
    D: Into<CompositeValue>,
    V: Into<CompositeValue>,
    N: Into<CompositeValue>,
{
    fn from(v: (D, V, N)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeRiver {
    #[serde(rename = "type")]
    type_: String,

    id: Option<String>,

    name: Option<String>,

    color_by: Option<ColorBy>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,

    width: Option<CompositeValue>,

    height: Option<CompositeValue>,

    coordinate_system: Option<CoordinateSystem>,

    boundary_gap: Option<BoundaryGap>,

    label: Option<Label>,


    data: Vec<ThemeRiverData>,
}

impl ThemeRiver {
    pub fn new() -> Self {
        Self {
            type_: "themeRiver".to_string(),
            id: None,
            name: None,
            color_by: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            coordinate_system: None,
            boundary_gap: None,
            label: None,
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

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<C: Into<CompositeValue>>(mut self, width: C) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<C: Into<CompositeValue>>(mut self, height: C) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn boundary_gap<B: Into<BoundaryGap>>(mut self, boundary_gap: B) -> Self {
        self.boundary_gap = Some(boundary_gap.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn data<T: Into<ThemeRiverData>>(mut self, data: Vec<T>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
