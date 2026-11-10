#[derive(Clone, Copy, Debug)]
pub struct SharePermission;

impl SharePermission {
    pub const GROUP: crate::Field<Self, crate::Object<super::GroupName>> =
        crate::Field::new("group");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const PROJECT: crate::Field<Self, crate::Object<super::Project>> =
        crate::Field::new("project");

    pub const ROLE: crate::Field<Self, crate::Object<super::ProjectRole>> =
        crate::Field::new("role");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USER: crate::Field<Self, crate::Object<super::UserBean>> = crate::Field::new("user");
}
