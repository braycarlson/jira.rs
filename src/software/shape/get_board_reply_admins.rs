#[derive(Clone, Copy, Debug)]
pub struct GetBoardReplyAdmins;

impl GetBoardReplyAdmins {
    pub const GROUPS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetBoardReplyAdminsGroups>>,
    > = crate::Field::new("groups");

    pub const USERS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetBoardReplyAdminsUsers>>,
    > = crate::Field::new("users");
}
