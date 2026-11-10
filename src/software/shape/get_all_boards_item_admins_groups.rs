#[derive(Clone, Copy, Debug)]
pub struct GetAllBoardsItemAdminsGroups;

impl GetAllBoardsItemAdminsGroups {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
