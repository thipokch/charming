use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::datatype::CompositeValue;

use super::{Color, Formatter, Padding, TextStyle, Trigger};

#[serde_auto]
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
