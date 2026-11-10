#[derive(Clone, Copy, Debug)]
pub struct CreateBoardReplyAdmins;

impl CreateBoardReplyAdmins {
    pub const GROUPS: crate::Field<
        Self,
        crate::List<crate::Object<super::CreateBoardReplyAdminsGroups>>,
    > = crate::Field::new("groups");

    pub const USERS: crate::Field<
        Self,
        crate::List<crate::Object<super::CreateBoardReplyAdminsUsers>>,
    > = crate::Field::new("users");
}
