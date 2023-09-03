use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{datatype::CompositeValue, element::Orient};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveAsImageType {
    Png,
    Jpg,
    Svg,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAsImage {
    show: Option<bool>,

    #[serde(rename = "type")]
    type_: Option<SaveAsImageType>,

    name: Option<String>,

    background_color: Option<String>,
}

impl SaveAsImage {
    pub fn new() -> Self {
        Self {
            show: None,
            type_: None,
            name: None,
            background_color: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn type_(mut self, type_: SaveAsImageType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn background_color<S: Into<String>>(mut self, background_color: S) -> Self {
        self.background_color = Some(background_color.into());
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restore {
    show: Option<bool>,

    title: Option<String>,
}

impl Restore {
    pub fn new() -> Self {
        Self {
            show: None,
            title: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataView {
    show: Option<bool>,

    title: Option<String>,

    read_only: Option<bool>,
}

impl DataView {
    pub fn new() -> Self {
        Self {
            show: None,
            title: None,
            read_only: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MagicTypeType {
    /// For line charts.
    Line,
    /// For bar charts.
    Bar,
    /// For stacked charts.
    Stack,
}

impl From<&str> for MagicTypeType {
    fn from(s: &str) -> Self {
        match s {
            "line" => Self::Line,
            "bar" => Self::Bar,
            "stack" => Self::Stack,
            _ => panic!("Invalid magic type type: {}", s),
        }
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicType {
    type_: Option<Vec<MagicTypeType>>,

    title: Option<String>,
}

impl MagicType {
    pub fn new() -> Self {
        Self {
            type_: None,
            title: None,
        }
    }

    pub fn type_(mut self, type_: Vec<MagicTypeType>) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BrushType {
    Rect,
    Polygon,
    LineX,
    LineY,
    Keep,
    Clear,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Brush {

    type_: Vec<BrushType>,
}

impl Brush {
    pub fn new() -> Self {
        Self { type_: vec![] }
    }

    pub fn type_(mut self, type_: Vec<BrushType>) -> Self {
        self.type_ = type_;
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxDataZoom {
    y_axis_index: Option<CompositeValue>,
}

impl ToolboxDataZoom {
    pub fn new() -> Self {
        Self { y_axis_index: None }
    }

    pub fn y_axis_index<C: Into<CompositeValue>>(mut self, y_axis_index: C) -> Self {
        self.y_axis_index = Some(y_axis_index.into());
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    save_as_image: Option<SaveAsImage>,

    restore: Option<Restore>,

    data_view: Option<DataView>,

    magic_type: Option<MagicType>,

    data_zoom: Option<ToolboxDataZoom>,

    brush: Option<Brush>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            save_as_image: None,
            restore: None,
            data_view: None,
            magic_type: None,
            data_zoom: None,
            brush: None,
        }
    }

    pub fn save_as_image(mut self, save_as_image: SaveAsImage) -> Self {
        self.save_as_image = Some(save_as_image);
        self
    }

    pub fn restore(mut self, restore: Restore) -> Self {
        self.restore = Some(restore);
        self
    }

    pub fn data_view(mut self, data_view: DataView) -> Self {
        self.data_view = Some(data_view);
        self
    }

    pub fn magic_type(mut self, magic_type: MagicType) -> Self {
        self.magic_type = Some(magic_type);
        self
    }

    pub fn data_zoom(mut self, data_zoom: ToolboxDataZoom) -> Self {
        self.data_zoom = Some(data_zoom);
        self
    }

    pub fn brush(mut self, brush: Brush) -> Self {
        self.brush = Some(brush);
        self
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toolbox {
    show: Option<bool>,

    feature: Option<Feature>,

    orient: Option<Orient>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self {
            show: None,
            feature: None,
            orient: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn feature<T: Into<Feature>>(mut self, feature: T) -> Self {
        self.feature = Some(feature.into());
        self
    }

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
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

    pub fn save_as_image_type(&self) -> Option<&SaveAsImageType> {
        self.feature
            .as_ref()
            .and_then(|f| f.save_as_image.as_ref())
            .and_then(|s| s.type_.as_ref())
    }
}
