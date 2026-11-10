#[derive(Clone, Copy, Debug)]
pub struct NotificationSchemePayload;

impl NotificationSchemePayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const NOTIFICATION_SCHEME_EVENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::NotificationSchemeEventPayload>>,
    > = crate::Field::new("notificationSchemeEvents");

    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
