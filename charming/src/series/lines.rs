use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::element::{ColorBy, CoordinateSystem, Emphasis, Label, LabelLayout, LineStyle, Symbol};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lines {
    // #[serde(rename = "type")]
    // type_: String,
    id: Option<String>,

    name: Option<String>,

    color_by: Option<ColorBy>,

    coordinate_system: Option<CoordinateSystem>,

    x_axis_index: Option<f64>,

    y_axis_index: Option<f64>,

    geo_index: Option<f64>,

    polyline: Option<bool>,

    large: Option<bool>,

    large_threshold: Option<f64>,

    symbol: Option<Symbol>,

    symbol_size: Option<f64>,

    line_style: Option<LineStyle>,

    label: Option<Label>,

    label_layout: Option<LabelLayout>,

    emphasis: Option<Emphasis>,
}
