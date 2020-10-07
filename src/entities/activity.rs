use serde::{Deserialize, Serialize};

/// Represents a weekly bucket of instance activity.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Activity {
    /// Midnight at the first day of the week.
    pub week: String,
    /// Statuses created since the week began.
    pub statuses: String,
    /// User logins since the week began.
    pub logins: String,
    /// User registrations since the week began.
    pub registrations: String,
}
