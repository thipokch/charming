use serde::{Deserialize, Serialize};
use macros::serde_auto;

#[serde_auto]
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
