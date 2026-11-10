#[derive(Clone, Copy, Debug)]
pub struct GetAllBoardsReplyValuesAdminsGroups;

impl GetAllBoardsReplyValuesAdminsGroups {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
