use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitArea {
    show: Option<bool>,
}

impl SplitArea {
    pub fn new() -> Self {
        Self { show: None }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }
}
