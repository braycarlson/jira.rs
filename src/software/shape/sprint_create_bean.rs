#[derive(Clone, Copy, Debug)]
pub struct SprintCreateBean;

impl SprintCreateBean {
    pub const END_DATE: crate::Field<Self, crate::Text> = crate::Field::new("endDate");
    pub const GOAL: crate::Field<Self, crate::Text> = crate::Field::new("goal");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ORIGIN_BOARD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("originBoardId");

    pub const START_DATE: crate::Field<Self, crate::Text> = crate::Field::new("startDate");
}
