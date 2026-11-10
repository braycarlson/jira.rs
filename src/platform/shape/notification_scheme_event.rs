#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeEvent;

impl NotificationSchemeEvent {
    pub const EVENT: crate::Field<Self, crate::Object<super::NotificationEvent>> =
        crate::Field::new("event");

    pub const NOTIFICATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::EventNotification>>,
    > = crate::Field::new("notifications");
}
