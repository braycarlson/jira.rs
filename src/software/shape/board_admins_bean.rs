#[derive(Clone, Copy, Debug)]
pub struct BoardAdminsBean;

impl BoardAdminsBean {
    pub const GROUPS: crate::Field<Self, crate::List<crate::Object<super::BoardAdminsBeanGroups>>> =
        crate::Field::new("groups");

    pub const USERS: crate::Field<Self, crate::List<crate::Object<super::BoardAdminsBeanUsers>>> =
        crate::Field::new("users");
}
