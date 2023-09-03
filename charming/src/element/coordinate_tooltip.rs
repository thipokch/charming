use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::datatype::CompositeValue;

use super::{Color, Formatter, Padding, TextStyle, Trigger};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateTooltip {
    show: Option<bool>,

    trigger: Option<Trigger>,

    position: Option<CompositeValue>,

    formatter: Option<Formatter>,

    value_formatter: Option<Formatter>,

    background_color: Option<Color>,

    border_color: Option<Color>,

    padding: Option<Padding>,

    text_style: Option<TextStyle>,
}
