#[derive(Clone, Copy, Debug)]
pub struct GetAllBoardsItemAdmins;

impl GetAllBoardsItemAdmins {
    pub const GROUPS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetAllBoardsItemAdminsGroups>>,
    > = crate::Field::new("groups");

    pub const USERS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetAllBoardsItemAdminsUsers>>,
    > = crate::Field::new("users");
}
