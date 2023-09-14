use serde::{Deserialize, Serialize};
use macros::serde_auto;

use super::RawString;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SymbolSize {
    Number(f64),
    Function(RawString),
}

impl From<i64> for SymbolSize {
    fn from(n: i64) -> Self {
        SymbolSize::Number(n as f64)
    }
}

impl From<f64> for SymbolSize {
    fn from(n: f64) -> Self {
        SymbolSize::Number(n)
    }
}

impl From<&str> for SymbolSize {
    fn from(s: &str) -> Self {
        SymbolSize::Function(RawString::from(s))
    }
}
