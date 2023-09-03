use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Sort {
    Ascending,
    Descending,
    None,
}

impl Serialize for Sort {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Sort::Ascending => serializer.serialize_str("ascending"),
            Sort::Descending => serializer.serialize_str("descending"),
            Sort::None => serializer.serialize_none(),
        }
    }
}
