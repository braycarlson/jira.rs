#[derive(Clone, Copy, Debug)]
pub struct NotificationRecipientsRestrictions;

impl NotificationRecipientsRestrictions {
    pub const GROUPS: crate::Field<Self, crate::List<crate::Object<super::GroupName>>> =
        crate::Field::new("groups");

    pub const GROUP_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("groupIds");

    pub const PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::RestrictedPermission>>,
    > = crate::Field::new("permissions");
}
