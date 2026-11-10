#[derive(Clone, Copy, Debug)]
pub struct GetAllBoardsReplyValuesAdmins;

impl GetAllBoardsReplyValuesAdmins {
    pub const GROUPS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetAllBoardsReplyValuesAdminsGroups>>,
    > = crate::Field::new("groups");

    pub const USERS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetAllBoardsReplyValuesAdminsUsers>>,
    > = crate::Field::new("users");
}
