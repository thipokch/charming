use serde::{Deserialize, Serialize};
use macros::serde_auto;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid3D {}

impl Grid3D {
    pub fn new() -> Self {
        Self {}
    }
}
