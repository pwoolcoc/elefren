use crate::entities::{notification::Notification, status::Status};

#[derive(Debug, Clone)]
/// Events that come from the /streaming/user API call
pub enum Event {
    /// Update event
    Update(Status),
    /// Notification event
    Notification(Notification),
    /// Delete event
    Delete(String),
    /// FiltersChanged event
    FiltersChanged,
}
