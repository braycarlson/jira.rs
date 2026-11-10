#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeEventPayload;

impl NotificationSchemeEventPayload {
    pub const EVENT: crate::Field<Self, crate::Object<super::NotificationSchemeEventIDPayload>> =
        crate::Field::new("event");

    pub const NOTIFICATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::NotificationSchemeNotificationDetailsPayload>>,
    > = crate::Field::new("notifications");
}
