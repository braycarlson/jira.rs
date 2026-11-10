#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeEventDetails;

impl NotificationSchemeEventDetails {
    pub const EVENT: crate::Field<Self, crate::Object<super::NotificationSchemeEventTypeID>> =
        crate::Field::new("event");

    pub const NOTIFICATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::NotificationSchemeNotificationDetails>>,
    > = crate::Field::new("notifications");
}
