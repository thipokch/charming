use serde::{Deserialize, Serialize};
use macros::serde_auto;

#[serde_auto]
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
