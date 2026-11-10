#[derive(Clone, Copy, Debug)]
pub struct NotificationRecipients;

impl NotificationRecipients {
    pub const ASSIGNEE: crate::Field<Self, crate::Boolean> = crate::Field::new("assignee");

    pub const GROUPS: crate::Field<Self, crate::List<crate::Object<super::GroupName>>> =
        crate::Field::new("groups");

    pub const GROUP_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("groupIds");

    pub const REPORTER: crate::Field<Self, crate::Boolean> = crate::Field::new("reporter");

    pub const USERS: crate::Field<Self, crate::List<crate::Object<super::UserDetails>>> =
        crate::Field::new("users");

    pub const VOTERS: crate::Field<Self, crate::Boolean> = crate::Field::new("voters");
    pub const WATCHERS: crate::Field<Self, crate::Boolean> = crate::Field::new("watchers");
}
