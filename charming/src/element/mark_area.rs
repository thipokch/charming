use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{blur::Blur, emphasis::Emphasis, item_style::ItemStyle, label::Label};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkAreaData {
    name: Option<String>,

    x_axis: Option<String>,

    y_axis: Option<String>,
}

impl MarkAreaData {
    pub fn new() -> Self {
        Self {
            name: None,
            x_axis: None,
            y_axis: None,
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn x_axis<F: Into<String>>(mut self, x_axis: F) -> Self {
        self.x_axis = Some(x_axis.into());
        self
    }

    pub fn y_axis<F: Into<String>>(mut self, y_axis: F) -> Self {
        self.y_axis = Some(y_axis.into());
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
pub struct MarkArea {
    silent: Option<bool>,

    label: Option<Label>,

    item_style: Option<ItemStyle>,

    emphasis: Option<Emphasis>,

    blur: Option<Blur>,


    data: Vec<(MarkAreaData, MarkAreaData)>,
}

impl MarkArea {
    pub fn new() -> Self {
        Self {
            silent: None,
            label: None,
            item_style: None,
            emphasis: None,
            blur: None,
            data: vec![],
        }
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn blur<B: Into<Blur>>(mut self, blur: B) -> Self {
        self.blur = Some(blur.into());
        self
    }

    pub fn data<D: Into<MarkAreaData>>(mut self, data: Vec<(D, D)>) -> Self {
        self.data = data
            .into_iter()
            .map(|(d1, d2)| (d1.into(), d2.into()))
            .collect();
        self
    }
}
