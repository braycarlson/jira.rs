#[derive(Clone, Copy, Debug)]
pub struct SprintBean;

impl SprintBean {
    pub const COMPLETE_DATE: crate::Field<Self, crate::Text> = crate::Field::new("completeDate");
    pub const CREATED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("createdDate");
    pub const END_DATE: crate::Field<Self, crate::Text> = crate::Field::new("endDate");
    pub const GOAL: crate::Field<Self, crate::Text> = crate::Field::new("goal");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ORIGIN_BOARD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("originBoardId");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const START_DATE: crate::Field<Self, crate::Text> = crate::Field::new("startDate");
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
}
