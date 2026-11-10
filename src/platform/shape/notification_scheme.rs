#[derive(Clone, Copy, Debug)]
pub struct NotificationScheme;

impl NotificationScheme {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const NOTIFICATION_SCHEME_EVENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::NotificationSchemeEvent>>,
    > = crate::Field::new("notificationSchemeEvents");

    pub const PROJECTS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projects");

    pub const SCOPE: crate::Field<Self, crate::Object<super::Scope>> = crate::Field::new("scope");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
