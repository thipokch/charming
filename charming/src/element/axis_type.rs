use serde::{Deserialize, Serialize};
use macros::serde_auto;

/// Type of axis.
#[serde_auto]
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
