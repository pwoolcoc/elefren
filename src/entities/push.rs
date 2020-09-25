use serde::{Deserialize, Serialize};

/// Represents the `alerts` key of the `Subscription` object
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Alerts {
    /// flag for follow alerts
    pub follow: Option<bool>,
    /// flag for favourite alerts
    pub favourite: Option<bool>,
    /// flag for reblog alerts
    pub reblog: Option<bool>,
    /// flag for mention alerts
    pub mention: Option<bool>,
}

/// Represents a new Push subscription
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Subscription {
    /// The `id` of the subscription
    pub id: String,
    /// The endpoint of the subscription
    pub endpoint: String,
    /// The server key of the subscription
    pub server_key: String,
    /// The status of the alerts for this subscription
    pub alerts: Option<Alerts>,
}

pub(crate) mod add_subscription {
    use serde::Serialize;
    use super::Alerts;

    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub(crate) struct Form {
        pub(crate) subscription: Subscription,
        pub(crate) data: Option<Data>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub(crate) struct Subscription {
        pub(crate) endpoint: String,
        pub(crate) keys: Keys,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub(crate) struct Keys {
        pub(crate) p256dh: String,
        pub(crate) auth: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub(crate) struct Data {
        pub(crate) alerts: Option<Alerts>,
    }
}

pub(crate) mod update_data {
    use serde::Serialize;
    use super::Alerts;

    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub(crate) struct Data {
        pub(crate) alerts: Option<Alerts>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub(crate) struct Form {
        pub(crate) id: String,
        pub(crate) data: Data,
    }
}
