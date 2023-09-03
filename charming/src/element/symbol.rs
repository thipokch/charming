use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq,  Deserialize)]
pub enum Symbol {
    Circle,
    Rect,
    RoundRect,
    Triangle,
    Diamond,
    Pin,
    Arrow,
    None,
    Custom(String),
}

impl Serialize for Symbol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Symbol::Circle => serializer.serialize_str("circle"),
            Symbol::Rect => serializer.serialize_str("rect"),
            Symbol::RoundRect => serializer.serialize_str("roundRect"),
            Symbol::Triangle => serializer.serialize_str("triangle"),
            Symbol::Diamond => serializer.serialize_str("diamond"),
            Symbol::Pin => serializer.serialize_str("pin"),
            Symbol::Arrow => serializer.serialize_str("arrow"),
            Symbol::None => serializer.serialize_str("none"),
            Symbol::Custom(s) => serializer.serialize_str(s),
        }
    }
}
