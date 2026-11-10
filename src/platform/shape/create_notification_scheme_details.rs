#[derive(Clone, Copy, Debug)]
pub struct CreateNotificationSchemeDetails;

impl CreateNotificationSchemeDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const NOTIFICATION_SCHEME_EVENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::NotificationSchemeEventDetails>>,
    > = crate::Field::new("notificationSchemeEvents");
}
