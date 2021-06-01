//! A module about contexts of statuses.

/// A context of a status returning a list of statuses it replied to and
/// statuses replied to it.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Context {
    /// Statuses that were replied to.
    pub ancestors: Vec<Status>,
    /// Statuses that replied to this status.
    pub descendants: Vec<Status>,
}

use serde::{Deserialize, Serialize};
use crate::status::Status;
