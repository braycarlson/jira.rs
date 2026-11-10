#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeNotificationDetailsPayload;

impl NotificationSchemeNotificationDetailsPayload {
    pub const NOTIFICATION_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("notificationType");

    pub const PARAMETER: crate::Field<Self, crate::Text> = crate::Field::new("parameter");
}
