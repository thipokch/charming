use serde::{Deserialize, Serialize};
use macros::serde_auto;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}
