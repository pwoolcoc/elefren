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

/// Entities for adding a push subscription
pub mod add_subscription {
    use super::Alerts;
    use serde::Serialize;

    /// TODO
    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub struct Form {
        /// TODO
        pub subscription: Subscription,
        /// TODO
        pub data: Option<Data>,
    }

    /// TODO
    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub struct Subscription {
        /// TODO
        pub endpoint: String,
        /// TODO
        pub keys: Keys,
    }

    /// TODO
    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub struct Keys {
        /// TODO
        pub p256dh: String,
        /// TODO
        pub auth: String,
    }

    /// TODO
    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub struct Data {
        /// TODO
        pub alerts: Option<Alerts>,
    }
}

/// Entities for push updates
pub mod update_data {
    use super::Alerts;
    use serde::Serialize;

    /// TODO
    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub struct Data {
        /// TODO
        pub alerts: Option<Alerts>,
    }

    /// TODO
    #[derive(Debug, Clone, PartialEq, Serialize, Default)]
    pub struct Form {
        /// TODO
        pub id: String,
        /// TODO
        pub data: Data,
    }
}
