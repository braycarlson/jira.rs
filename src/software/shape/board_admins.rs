#[derive(Clone, Copy, Debug)]
pub struct BoardAdmins;

impl BoardAdmins {
    pub const GROUPS: crate::Field<Self, crate::List<crate::Object<super::BoardAdminsGroups>>> =
        crate::Field::new("groups");

    pub const USERS: crate::Field<Self, crate::List<crate::Object<super::BoardAdminsUsers>>> =
        crate::Field::new("users");
}
