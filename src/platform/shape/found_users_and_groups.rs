#[derive(Clone, Copy, Debug)]
pub struct FoundUsersAndGroups;

impl FoundUsersAndGroups {
    pub const GROUPS: crate::Field<Self, crate::Object<super::FoundGroups>> =
        crate::Field::new("groups");

    pub const USERS: crate::Field<Self, crate::Object<super::FoundUsers>> =
        crate::Field::new("users");
}
