#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// Admin-level information about a given account.
pub struct Account {
    id: String,
    username: String,
    domain: String,
    created_at: DateTime<Utc>,
    email: String,
    ip: String,
    locale: Language,
    invite_request: String,
    role: String, // TODO: Docs says it is an enum, need to check on the variants
    confirmed: bool,
    approved: bool,
    disabled: bool,
    silenced: bool,
    suspended: bool,
    account: crate::account::Account,
    created_by_application_id: Option<String>,
    invited_by_account_id: Option<String>,

    /// A place that unknown fields go. This is mainly provided for forwards compatibility,
    /// i.e. if you want to support mastodon versions going back to 2.4.0 but don't want deser
    /// errors with newer versions, you can set `--no-default-features --features mastodon_2_4_0`
    /// and newer fields will go here so they can still be used
    #[serde(flatten)]
    elefren_extra: HashMap<String, Value>,
}
impl Account {
    /// The ID of the account in the database.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// The username of the account.
    pub fn username(&self) -> &str {
        &self.username
    }
    /// The domain of the account.
    pub fn domain(&self) -> &str {
        &self.domain
    }
    /// When the account was first discovered.
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    /// The email address associated with the account.
    pub fn email(&self) -> &str {
        &self.email
    }
    /// The IP address last used to login to this account.
    pub fn ip(&self) -> &str {
        &self.ip
    }
    /// The locale of the account.
    pub fn locale(&self) -> &Language {
        &self.locale
    }
    /// Invite request text
    pub fn invite_request(&self) -> &str {
        &self.invite_request
    }
    /// The current role of the account.
    pub fn role(&self) -> &str {
        &self.role
    }
    /// Whether the account has confirmed their email address.
    pub fn confirmed(&self) -> bool {
        self.confirmed
    }
    /// Whether the account is currently approved.
    pub fn approved(&self) -> bool {
        self.approved
    }
    /// Whether the account is currently disabled.
    pub fn disabled(&self) -> bool {
        self.disabled
    }
    /// Whether the account is currently silenced.
    pub fn silenced(&self) -> bool {
        self.silenced
    }
    /// Whether the account is currently suspended.
    pub fn suspended(&self) -> bool {
        self.suspended
    }
    /// User-level information about the account.
    pub fn account(&self) -> &crate::account::Account {
        &self.account
    }
    /// The ID of the application that created this account.
    pub fn created_by_application_id(&self) -> Option<&String> {
        self.created_by_application_id.as_ref()
    }
    /// The ID of the account that invited this user
    pub fn invited_by_account_id(&self) -> Option<&String> {
        self.invited_by_account_id.as_ref()
    }
}

use chrono::{DateTime, Utc};
use isolang::Language;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
