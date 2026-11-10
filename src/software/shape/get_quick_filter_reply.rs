#[derive(Clone, Copy, Debug)]
pub struct GetQuickFilterReply;

impl GetQuickFilterReply {
    pub const BOARD_ID: crate::Field<Self, crate::Integer> = crate::Field::new("boardId");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");
}
