use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::line_style::LineStyle;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinorSplitLine {
    show: Option<bool>,
    line_style: Option<LineStyle>,
}

impl MinorSplitLine {
    pub fn new() -> MinorSplitLine {
        MinorSplitLine {
            show: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> MinorSplitLine {
        self.show = Some(show);
        self
    }

    pub fn line_style<F: Into<LineStyle>>(mut self, line_style: F) -> MinorSplitLine {
        self.line_style = Some(line_style.into());
        self
    }
}
