use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::RawString;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
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
