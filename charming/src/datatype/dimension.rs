use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DimensionType {
    Number,
    Float,
    Int,
    Ordinal,
    Time,
}

impl From<&str> for DimensionType {
    fn from(s: &str) -> Self {
        match s {
            "number" => Self::Number,
            "float" => Self::Float,
            "int" => Self::Int,
            "ordinal" => Self::Ordinal,
            "time" => Self::Time,
            _ => panic!("Invalid dimension type: {}", s),
        }
    }
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(rename = "type")]
    type_: Option<DimensionType>,

    name: Option<String>,

    display_name: Option<String>,
}

impl Dimension {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            display_name: None,
        }
    }

    pub fn type_<D: Into<DimensionType>>(mut self, type_: D) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn display_name<S: Into<String>>(mut self, display_name: S) -> Self {
        self.display_name = Some(display_name.into());
        self
    }
}

impl From<&str> for Dimension {
    fn from(name: &str) -> Self {
        Self::new().name(name)
    }
}

impl From<(&str, &str)> for Dimension {
    fn from((name, type_): (&str, &str)) -> Self {
        Self::new().name(name).type_(type_)
    }
}

impl From<(&str, &str, &str)> for Dimension {
    fn from((name, type_, display_name): (&str, &str, &str)) -> Self {
        Self::new()
            .name(name)
            .type_(type_)
            .display_name(display_name)
    }
}

#[macro_export]
macro_rules! dim {
    ($($x:expr),*) => {
        vec![
            $(
                $crate::datatype::Dimension::from($x)
            ),*
        ]
    };
}
