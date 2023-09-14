use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::RawString;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Formatter {
    String(String),
    Function(RawString),
}

impl From<&str> for Formatter {
    fn from(s: &str) -> Self {
        Formatter::String(s.to_string())
    }
}

impl From<RawString> for Formatter {
    fn from(s: RawString) -> Self {
        Formatter::Function(s)
    }
}
