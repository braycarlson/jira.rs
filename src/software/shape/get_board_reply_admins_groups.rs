#[derive(Clone, Copy, Debug)]
pub struct GetBoardReplyAdminsGroups;

impl GetBoardReplyAdminsGroups {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
