use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    datatype::CompositeValue,
    element::{Blur, Emphasis, ItemStyle, Label, Select, Symbol},
};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeLayout {
    Orthogonal,
    Radial,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TreeOrient {
    #[serde(rename = "LR")]
    LeftRight,
    #[serde(rename = "RL")]
    RightLeft,
    #[serde(rename = "TB")]
    TopBottom,
    #[serde(rename = "BT")]
    BottomTop,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeEdgeShape {
    Curve,
    Polyline,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeLeaves {
    label: Option<Label>,
}

impl TreeLeaves {
    pub fn new() -> Self {
        Self { label: None }
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
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
pub struct TreeNode {
    pub name: String,

    pub value: Option<f64>,

    pub collapsed: Option<bool>,

    pub children: Option<Vec<TreeNode>>,
}

/// The tree diagram is mainly used to display the tree data structure.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    #[serde(rename = "type")]
    type_: String,

    /// Component ID.
    id: Option<String>,

    /// Component name.
    name: Option<String>,

    /// zlevel value of all graphical elements in the tree.
    z_level: Option<u64>,

    /// z value of all graphical elements in the tree.
    z: Option<u64>,

    left: Option<CompositeValue>,

    top: Option<CompositeValue>,

    right: Option<CompositeValue>,

    bottom: Option<CompositeValue>,

    width: Option<CompositeValue>,

    height: Option<CompositeValue>,

    center: Option<CompositeValue>,

    zoom: Option<f64>,

    layout: Option<TreeLayout>,

    orient: Option<TreeOrient>,

    symbol: Option<Symbol>,

    symbol_size: Option<f64>,

    symbol_rotate: Option<f64>,

    symbol_keep_aspect: Option<bool>,

    symbol_offset: Option<CompositeValue>,

    edge_shape: Option<TreeEdgeShape>,

    edge_fork_position: Option<String>,

    roam: Option<bool>,

    initial_tree_depth: Option<f64>,

    item_style: Option<ItemStyle>,

    label: Option<Label>,

    emphasis: Option<Emphasis>,

    blur: Option<Blur>,

    select: Option<Select>,

    selected_mode: Option<bool>,

    expand_and_collapse: Option<bool>,

    animation_duration: Option<f64>,

    animation_duration_update: Option<f64>,

    leaves: Option<TreeLeaves>,


    data: Vec<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            type_: "tree".into(),
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
            center: None,
            zoom: None,
            layout: None,
            orient: None,
            symbol: None,
            symbol_size: None,
            symbol_rotate: None,
            symbol_keep_aspect: None,
            symbol_offset: None,
            edge_shape: None,
            edge_fork_position: None,
            roam: None,
            initial_tree_depth: None,
            item_style: None,
            label: None,
            emphasis: None,
            blur: None,
            select: None,
            selected_mode: None,
            expand_and_collapse: None,
            animation_duration: None,
            animation_duration_update: None,
            leaves: None,
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

    pub fn center<C: Into<CompositeValue>>(mut self, center: C) -> Self {
        self.center = Some(center.into());
        self
    }

    pub fn zoom<F: Into<f64>>(mut self, zoom: F) -> Self {
        self.zoom = Some(zoom.into());
        self
    }

    pub fn layout<T: Into<TreeLayout>>(mut self, layout: T) -> Self {
        self.layout = Some(layout.into());
        self
    }

    pub fn orient<T: Into<TreeOrient>>(mut self, orient: T) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn symbol<S: Into<Symbol>>(mut self, symbol: S) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn symbol_size<F: Into<f64>>(mut self, symbol_size: F) -> Self {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn symbol_rotate<F: Into<f64>>(mut self, symbol_rotate: F) -> Self {
        self.symbol_rotate = Some(symbol_rotate.into());
        self
    }

    pub fn symbol_keep_aspect(mut self, symbol_keep_aspect: bool) -> Self {
        self.symbol_keep_aspect = Some(symbol_keep_aspect);
        self
    }

    pub fn symbol_offset<C: Into<CompositeValue>>(mut self, symbol_offset: C) -> Self {
        self.symbol_offset = Some(symbol_offset.into());
        self
    }

    pub fn edge_shape<T: Into<TreeEdgeShape>>(mut self, edge_shape: T) -> Self {
        self.edge_shape = Some(edge_shape.into());
        self
    }

    pub fn edge_fork_position<S: Into<String>>(mut self, edge_fork_position: S) -> Self {
        self.edge_fork_position = Some(edge_fork_position.into());
        self
    }

    pub fn roam(mut self, roam: bool) -> Self {
        self.roam = Some(roam);
        self
    }

    pub fn initial_tree_depth<F: Into<f64>>(mut self, initial_tree_depth: F) -> Self {
        self.initial_tree_depth = Some(initial_tree_depth.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
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

    pub fn select<S: Into<Select>>(mut self, select: S) -> Self {
        self.select = Some(select.into());
        self
    }

    pub fn selected_mode(mut self, selected_mode: bool) -> Self {
        self.selected_mode = Some(selected_mode);
        self
    }

    pub fn expand_and_collapse(mut self, expand_and_collapse: bool) -> Self {
        self.expand_and_collapse = Some(expand_and_collapse);
        self
    }

    pub fn animation_duration<F: Into<f64>>(mut self, animation_duration: F) -> Self {
        self.animation_duration = Some(animation_duration.into());
        self
    }

    pub fn animation_duration_update<F: Into<f64>>(mut self, animation_duration_update: F) -> Self {
        self.animation_duration_update = Some(animation_duration_update.into());
        self
    }

    pub fn leaves<T: Into<TreeLeaves>>(mut self, leaves: T) -> Self {
        self.leaves = Some(leaves.into());
        self
    }

    pub fn data<T: Into<TreeNode>>(mut self, data: Vec<T>) -> Self {
        self.data = data.into_iter().map(|t| t.into()).collect();
        self
    }
}
