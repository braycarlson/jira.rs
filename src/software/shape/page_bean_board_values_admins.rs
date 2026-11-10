#[derive(Clone, Copy, Debug)]
pub struct PageBeanBoardValuesAdmins;

impl PageBeanBoardValuesAdmins {
    pub const GROUPS: crate::Field<
        Self,
        crate::List<crate::Object<super::PageBeanBoardValuesAdminsGroups>>,
    > = crate::Field::new("groups");

    pub const USERS: crate::Field<
        Self,
        crate::List<crate::Object<super::PageBeanBoardValuesAdminsUsers>>,
    > = crate::Field::new("users");
}
