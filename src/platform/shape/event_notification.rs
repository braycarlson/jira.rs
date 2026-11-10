#[derive(Clone, Copy, Debug)]
pub struct EventNotification;

impl EventNotification {
    pub const EMAIL_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("emailAddress");
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const FIELD: crate::Field<Self, crate::Object<super::FieldDetails>> =
        crate::Field::new("field");

    pub const GROUP: crate::Field<Self, crate::Object<super::GroupName>> =
        crate::Field::new("group");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const NOTIFICATION_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("notificationType");

    pub const PARAMETER: crate::Field<Self, crate::Text> = crate::Field::new("parameter");

    pub const PROJECT_ROLE: crate::Field<Self, crate::Object<super::ProjectRole>> =
        crate::Field::new("projectRole");

    pub const RECIPIENT: crate::Field<Self, crate::Text> = crate::Field::new("recipient");

    pub const USER: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("user");
}
