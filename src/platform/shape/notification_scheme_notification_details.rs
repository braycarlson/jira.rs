#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemeNotificationDetails;

impl NotificationSchemeNotificationDetails {
    pub const NOTIFICATION_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("notificationType");

    pub const PARAMETER: crate::Field<Self, crate::Text> = crate::Field::new("parameter");
}
