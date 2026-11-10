#[derive(Clone, Copy, Debug)]
pub struct FoundGroup;

impl FoundGroup {
    pub const AVATAR_URL: crate::Field<Self, crate::Text> = crate::Field::new("avatarUrl");
    pub const GROUP_ID: crate::Field<Self, crate::Text> = crate::Field::new("groupId");
    pub const HTML: crate::Field<Self, crate::Text> = crate::Field::new("html");

    pub const LABELS: crate::Field<Self, crate::List<crate::Object<super::GroupLabel>>> =
        crate::Field::new("labels");

    pub const MANAGED_BY: crate::Field<Self, crate::Text> = crate::Field::new("managedBy");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const USAGE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("usageType");
}
