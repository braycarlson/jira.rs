#[derive(Clone, Copy, Debug)]
pub struct ProjectRoleGroup;

impl ProjectRoleGroup {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const GROUP_ID: crate::Field<Self, crate::Text> = crate::Field::new("groupId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
