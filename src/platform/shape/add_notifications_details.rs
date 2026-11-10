#[derive(Clone, Copy, Debug)]
pub struct AddNotificationsDetails;

impl AddNotificationsDetails {
    pub const NOTIFICATION_SCHEME_EVENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::NotificationSchemeEventDetails>>,
    > = crate::Field::new("notificationSchemeEvents");
}
