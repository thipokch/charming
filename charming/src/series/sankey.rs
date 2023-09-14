use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::{
    datatype::CompositeValue,
    element::{Emphasis, Label, LineStyle, Orient},
};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SankeyNodeAlign {
    Left,
    Right,
    Justify,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SankeyNode {
    pub name: String,

    pub value: Option<f64>,

    pub depth: Option<f64>,
}

impl<S> From<S> for SankeyNode
where
    S: Into<String>,
{
    fn from(name: S) -> Self {
        SankeyNode {
            name: name.into(),
            value: None,
            depth: None,
        }
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: f64,
}

impl<S, F> From<(S, S, F)> for SankeyLink
where
    S: Into<String>,
    F: Into<f64>,
{
    fn from((source, target, value): (S, S, F)) -> Self {
        SankeyLink {
            source: source.into(),
            target: target.into(),
            value: value.into(),
        }
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sankey {
    id: Option<String>,

    name: Option<String>,

    z_level: Option<f64>,

    z: Option<f64>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,

    width: Option<f64>,

    height: Option<f64>,

    emphasis: Option<Emphasis>,

    orient: Option<Orient>,

    label: Option<Label>,

    node_align: Option<SankeyNodeAlign>,

    line_style: Option<LineStyle>,


    links: Vec<SankeyLink>,


    data: Vec<SankeyNode>,
}

impl Sankey {
    pub fn new() -> Self {
        Sankey {
            id: None,
            name: None,
            z_level: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            emphasis: None,
            orient: None,
            label: None,
            node_align: None,
            line_style: None,
            links: vec![],
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

    pub fn z_level<F: Into<f64>>(mut self, z_level: F) -> Self {
        self.z_level = Some(z_level.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
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

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<F: Into<f64>>(mut self, height: F) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn node_align<S: Into<SankeyNodeAlign>>(mut self, node_align: S) -> Self {
        self.node_align = Some(node_align.into());
        self
    }

    pub fn line_style<L: Into<LineStyle>>(mut self, line_style: L) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn data<S: Into<SankeyNode>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn nodes<S: Into<SankeyNode>>(mut self, nodes: Vec<S>) -> Self {
        self.data = nodes.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn links<S: Into<SankeyLink>>(mut self, links: Vec<S>) -> Self {
        self.links = links.into_iter().map(|s| s.into()).collect();
        self
    }
}
