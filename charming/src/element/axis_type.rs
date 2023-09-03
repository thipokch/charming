use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Type of axis.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(default, skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisType {
    /// Numerical axis, suitable for continuous data.
    Value,
    /// Category axis, suitable for discrete category data.
    Category,
    /// Time axis, suitable for continuous time series data.
    Time,
    /// Log axis, suitable for log data.
    Log,
}
