use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{ColorBy, Emphasis, ItemStyle, Label, LabelLine, Orient, Sort},
};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    Left,
    Right,
    Center,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Funnel {
    id: Option<String>,

    name: Option<String>,

    color_by: Option<ColorBy>,

    min: Option<f64>,

    max: Option<f64>,

    min_size: Option<String>,

    max_size: Option<String>,

    width: Option<CompositeValue>,

    height: Option<CompositeValue>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,

    orient: Option<Orient>,

    sort: Option<Sort>,

    gap: Option<f64>,

    legend_hover_link: Option<bool>,

    funnel_align: Option<Align>,

    label: Option<Label>,

    label_line: Option<LabelLine>,

    item_style: Option<ItemStyle>,

    emphasis: Option<Emphasis>,


    data: DataFrame,
}

impl Funnel {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            color_by: None,
            min: None,
            max: None,
            min_size: None,
            max_size: None,
            width: None,
            height: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            orient: None,
            sort: None,
            gap: None,
            legend_hover_link: None,
            funnel_align: None,
            label: None,
            label_line: None,
            item_style: None,
            emphasis: None,
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

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn min_size<S: Into<String>>(mut self, min_size: S) -> Self {
        self.min_size = Some(min_size.into());
        self
    }

    pub fn max_size<S: Into<String>>(mut self, max_size: S) -> Self {
        self.max_size = Some(max_size.into());
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

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn sort<S: Into<Sort>>(mut self, sort: S) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn gap<F: Into<f64>>(mut self, gap: F) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn funnel_align<A: Into<Align>>(mut self, funnel_align: A) -> Self {
        self.funnel_align = Some(funnel_align.into());
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

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
