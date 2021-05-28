/// Admin-level information about a filed report.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Report {
    id: String,
    action_taken: String, // TODO enumerable
    comment: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    account: crate::entities::account::Account,
    target_account: crate::entities::account::Account,
    assigned_account: crate::entities::account::Account,
    action_taken_by_account: String, // TODO enumerable
    statuses: Vec<crate::entities::status::Status>,

    /// A place that unknown fields go. This is mainly provided for forwards compatibility,
    /// i.e. if you want to support mastodon versions going back to 2.4.0 but don't want deser
    /// errors with newer versions, you can set `--no-default-features --features mastodon_2_4_0`
    /// and newer fields will go here so they can still be used
    #[serde(flatten)]
    elefren_extra_fields: HashMap<String, Value>,
}
impl Report {
    /// The ID of the report in the database.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// The action taken to resolve this report.
    pub fn action_taken(&self) -> &str {
        &self.action_taken
    }
    /// An optional reason for reporting.
    pub fn comment(&self) -> &str {
        &self.comment
    }
    /// The time the report was filed.
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    /// The time of last action on this report.
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    /// The account which filed the report.
    pub fn account(&self) -> &crate::entities::account::Account {
        &self.account
    }
    /// The account being reported.
    pub fn target_account(&self) -> &crate::entities::account::Account {
        &self.target_account
    }
    /// The account of the moderator assigned to this report.
    pub fn assigned_account(&self) -> &crate::entities::account::Account {
        &self.assigned_account
    }
    /// The action taken by the moderator who handled the report.
    pub fn action_taken_by_account(&self) -> &str {
        &self.action_taken_by_account
    }
    /// Statuses attached to the report, for context.
    pub fn statuses(&self) -> &[crate::entities::status::Status] {
        &self.statuses
    }
}

use chrono::{DateTime, Utc};
//use isolang::Language;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
